use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    User,
    Group,
    Application,
    ServicePrincipal,
    DirectoryRole,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            NodeType::User => "User",
            NodeType::Group => "Group",
            NodeType::Application => "Application",
            NodeType::ServicePrincipal => "ServicePrincipal",
            NodeType::DirectoryRole => "DirectoryRole",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    #[default]
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RiskLevel::Low => "Low",
            RiskLevel::Medium => "Medium",
            RiskLevel::High => "High",
            RiskLevel::Critical => "Critical",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for RiskLevel {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(RiskLevel::Low),
            "medium" => Ok(RiskLevel::Medium),
            "high" => Ok(RiskLevel::High),
            "critical" => Ok(RiskLevel::Critical),
            _ => Err(anyhow::anyhow!("Unknown risk level: {s}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    MemberOf,
    AppRoleAssignment,
    RoleAssignment,
    OAuthPermissionGrant,
    ServicePrincipalOf,
}

impl std::fmt::Display for EdgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EdgeType::MemberOf => "MemberOf",
            EdgeType::AppRoleAssignment => "AppRoleAssignment",
            EdgeType::RoleAssignment => "RoleAssignment",
            EdgeType::OAuthPermissionGrant => "OAuthPermissionGrant",
            EdgeType::ServicePrincipalOf => "ServicePrincipalOf",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub display_name: String,
    pub node_type: NodeType,
    pub risk_level: RiskLevel,
    pub properties: HashMap<String, serde_json::Value>,
}

impl Node {
    pub fn new(id: String, display_name: String, node_type: NodeType) -> Self {
        Self {
            id,
            display_name,
            node_type,
            risk_level: RiskLevel::Low,
            properties: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from_id: String,
    pub to_id: String,
    pub edge_type: EdgeType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccessGraph {
    pub nodes: HashMap<String, Node>,
    pub edges: Vec<Edge>,
}

impl AccessGraph {
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_edge(&mut self, from_id: String, to_id: String, edge_type: EdgeType) {
        if self.nodes.contains_key(&from_id) && self.nodes.contains_key(&to_id) {
            self.edges.push(Edge { from_id, to_id, edge_type });
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegeChain {
    pub path: Vec<String>,
    pub node_names: Vec<String>,
    pub risk_level: RiskLevel,
    pub description: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub critical_nodes: usize,
    pub high_nodes: usize,
    pub medium_nodes: usize,
    pub low_nodes: usize,
    pub total_chains: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RiskReport {
    pub summary: ReportSummary,
    pub high_risk_nodes: Vec<Node>,
    pub privilege_chains: Vec<PrivilegeChain>,
    /// Alle Nodes (nicht nur high-risk) mit node_type, und alle Edges
    /// (inkl. MemberOf) - fuer Konsumenten, die die volle Graphstruktur
    /// brauchen (z.B. eine User x Gruppe Mitgliedschafts-Matrix oder eine
    /// eigene Node-Link-Visualisierung), ohne selbst neu gegen Graph
    /// scannen zu muessen.
    #[serde(default)]
    pub all_nodes: Vec<Node>,
    #[serde(default)]
    pub edges: Vec<Edge>,
}
