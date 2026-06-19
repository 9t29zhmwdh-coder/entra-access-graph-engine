use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use eagraph_core::{
    chain_detector::find_privilege_chains,
    edge_analyzer::EntraGraph,
    exporter::{export_graphml, export_html, export_json},
    graph_client::GraphClient,
    model::{RiskLevel, RiskReport},
    node_builder::{build_access_graph, mock_access_graph},
    risk_scorer::{build_report_summary, score_graph},
};
use tracing::info;

#[derive(Parser)]
#[command(
    name = "eagraph",
    about = "Entra Access Graph Engine - Map Entra ID objects to privilege chains and risk scores",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Scan {
        #[arg(long, env = "AZURE_TENANT_ID", help = "Azure tenant ID")]
        tenant_id: Option<String>,

        #[arg(long, env = "AZURE_CLIENT_ID", help = "App registration client ID")]
        client_id: Option<String>,

        #[arg(long, env = "AZURE_CLIENT_SECRET", help = "App registration client secret")]
        client_secret: Option<String>,

        #[arg(long, default_value = "json", help = "Output format")]
        format: OutputFormat,

        #[arg(long, default_value = "eagraph-report", help = "Output file base name (without extension)")]
        output: PathBuf,

        #[arg(long, default_value = "high", help = "Minimum risk level to report")]
        min_risk: RiskLevelArg,

        #[arg(long, help = "Use mock data instead of live Graph API call")]
        dry_run: bool,
    },
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Json,
    Html,
    Graphml,
}

#[derive(Clone, ValueEnum)]
enum RiskLevelArg {
    Low,
    Medium,
    High,
    Critical,
}

impl From<RiskLevelArg> for RiskLevel {
    fn from(v: RiskLevelArg) -> Self {
        match v {
            RiskLevelArg::Low => RiskLevel::Low,
            RiskLevelArg::Medium => RiskLevel::Medium,
            RiskLevelArg::High => RiskLevel::High,
            RiskLevelArg::Critical => RiskLevel::Critical,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("eagraph=info".parse()?),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Command::Scan {
            tenant_id,
            client_id,
            client_secret,
            format,
            output,
            min_risk,
            dry_run,
        } => {
            run_scan(
                tenant_id,
                client_id,
                client_secret,
                format,
                output,
                RiskLevel::from(min_risk),
                dry_run,
            )
            .await
        }
    }
}

async fn run_scan(
    tenant_id: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    format: OutputFormat,
    output: PathBuf,
    min_risk: RiskLevel,
    dry_run: bool,
) -> Result<()> {
    let mut graph = if dry_run {
        info!("Dry-run mode: using mock graph data");
        mock_access_graph()
    } else {
        let tid = tenant_id.context("AZURE_TENANT_ID is required for live scan")?;
        let cid = client_id.context("AZURE_CLIENT_ID is required for live scan")?;
        let csecret = client_secret.context("AZURE_CLIENT_SECRET is required for live scan")?;
        let client = GraphClient::new(tid, cid, csecret);
        build_access_graph(&client).await?
    };

    info!("Scoring nodes...");
    score_graph(&mut graph);

    info!("Detecting privilege chains (min risk: {min_risk})...");
    let entra_graph = EntraGraph::build_from(&graph);
    let chains = find_privilege_chains(&entra_graph, &graph, &min_risk);
    info!("Found {} privilege chains", chains.len());

    let high_risk_nodes: Vec<_> = graph
        .nodes
        .values()
        .filter(|n| n.risk_level >= min_risk)
        .cloned()
        .collect();

    let report = RiskReport {
        summary: build_report_summary(&graph, &chains),
        high_risk_nodes,
        privilege_chains: chains,
    };

    let ext = match format {
        OutputFormat::Json => "json",
        OutputFormat::Html => "html",
        OutputFormat::Graphml => "graphml",
    };
    let out_path = output.with_extension(ext);

    match format {
        OutputFormat::Json => export_json(&report, &out_path)?,
        OutputFormat::Html => export_html(&report, &graph, &out_path)?,
        OutputFormat::Graphml => export_graphml(&graph, &out_path)?,
    }

    info!("Report written to {}", out_path.display());
    println!("Summary:");
    println!("  Nodes:    {}", report.summary.total_nodes);
    println!("  Critical: {}", report.summary.critical_nodes);
    println!("  High:     {}", report.summary.high_nodes);
    println!("  Chains:   {}", report.summary.total_chains);
    println!("  Output:   {}", out_path.display());

    Ok(())
}
