use std::path::Path;

use anyhow::Result;

use crate::model::{AccessGraph, RiskReport};

pub fn export_json(report: &RiskReport, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(report)?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn export_graphml(graph: &AccessGraph, path: &Path) -> Result<()> {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<graphml xmlns=\"http://graphml.graphdrawing.org/graphml\">\n");
    xml.push_str("  <key id=\"name\" for=\"node\" attr.name=\"name\" attr.type=\"string\"/>\n");
    xml.push_str("  <key id=\"type\" for=\"node\" attr.name=\"type\" attr.type=\"string\"/>\n");
    xml.push_str("  <key id=\"risk\" for=\"node\" attr.name=\"risk\" attr.type=\"string\"/>\n");
    xml.push_str("  <key id=\"etype\" for=\"edge\" attr.name=\"type\" attr.type=\"string\"/>\n");
    xml.push_str("  <graph id=\"G\" edgedefault=\"directed\">\n");

    for node in graph.nodes.values() {
        xml.push_str(&format!("    <node id=\"{}\">\n", xml_escape(&node.id)));
        xml.push_str(&format!(
            "      <data key=\"name\">{}</data>\n",
            xml_escape(&node.display_name)
        ));
        xml.push_str(&format!(
            "      <data key=\"type\">{}</data>\n",
            xml_escape(&node.node_type.to_string())
        ));
        xml.push_str(&format!(
            "      <data key=\"risk\">{}</data>\n",
            xml_escape(&node.risk_level.to_string())
        ));
        xml.push_str("    </node>\n");
    }

    for (i, edge) in graph.edges.iter().enumerate() {
        xml.push_str(&format!(
            "    <edge id=\"e{i}\" source=\"{}\" target=\"{}\">\n",
            xml_escape(&edge.from_id),
            xml_escape(&edge.to_id)
        ));
        xml.push_str(&format!(
            "      <data key=\"etype\">{}</data>\n",
            xml_escape(&edge.edge_type.to_string())
        ));
        xml.push_str("    </edge>\n");
    }

    xml.push_str("  </graph>\n</graphml>\n");
    std::fs::write(path, xml)?;
    Ok(())
}

pub fn export_html(report: &RiskReport, graph: &AccessGraph, path: &Path) -> Result<()> {
    let report_json = serde_json::to_string(report)?;
    let graph_json = serde_json::to_string(graph)?;
    let html = build_html(&report_json, &graph_json);
    std::fs::write(path, html)?;
    Ok(())
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn build_html(report_json: &str, graph_json: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Entra Access Graph Engine - Risk Report</title>
<style>
  *{{box-sizing:border-box;margin:0;padding:0}}
  body{{font-family:'Segoe UI',Tahoma,Geneva,Verdana,sans-serif;background:#f5f5f5;color:#323130}}
  header{{background:#0078d4;color:white;padding:16px 32px}}
  header h1{{font-size:22px;font-weight:600}}
  header p{{font-size:13px;opacity:.85;margin-top:4px}}
  main{{max-width:1280px;margin:0 auto;padding:24px 32px}}
  .grid{{display:grid;grid-template-columns:repeat(4,1fr);gap:16px;margin-bottom:28px}}
  .card{{background:white;border-radius:4px;padding:18px;box-shadow:0 1px 3px rgba(0,0,0,.1)}}
  .card h3{{font-size:13px;color:#605e5c;font-weight:400;margin-bottom:8px}}
  .num{{font-size:34px;font-weight:700}}
  .critical{{color:#d13438}}.high{{color:#d83b01}}.medium{{color:#ca5010}}.low{{color:#107c10}}
  #svg-wrap{{background:white;border-radius:4px;box-shadow:0 1px 3px rgba(0,0,0,.1);padding:16px;margin-bottom:28px}}
  #svg-wrap h2{{font-size:17px;margin-bottom:12px}}
  svg{{width:100%;height:520px;border:1px solid #edebe9;border-radius:2px}}
  .link{{stroke:#c8c6c4;stroke-opacity:.6;fill:none}}
  .node text{{font-size:11px;pointer-events:none}}
  h2.section{{font-size:18px;margin:28px 0 12px}}
  table{{width:100%;border-collapse:collapse;background:white;border-radius:4px;box-shadow:0 1px 3px rgba(0,0,0,.1)}}
  th{{background:#f3f2f1;padding:11px 16px;text-align:left;font-size:13px;font-weight:600}}
  td{{padding:9px 16px;border-bottom:1px solid #f3f2f1;font-size:13px}}
  tr:last-child td{{border-bottom:none}}
  .badge{{display:inline-block;padding:2px 10px;border-radius:12px;font-size:12px;font-weight:600}}
  .badge-critical{{background:#fde7e9;color:#d13438}}
  .badge-high{{background:#fed9cc;color:#d83b01}}
  .badge-medium{{background:#fff4ce;color:#ca5010}}
  .badge-low{{background:#dff6dd;color:#107c10}}
  .chain-card{{background:white;border-radius:4px;box-shadow:0 1px 3px rgba(0,0,0,.1);padding:14px 16px;margin-bottom:10px;border-left:4px solid}}
  .chain-card.critical{{border-left-color:#d13438}}
  .chain-card.high{{border-left-color:#d83b01}}
  .chain-card.medium{{border-left-color:#ca5010}}
  .chain-path{{font-family:Consolas,monospace;font-size:13px}}
  .chain-desc{{font-size:12px;color:#605e5c;margin-top:6px}}
  footer{{text-align:center;padding:22px;color:#605e5c;font-size:12px}}
  .legend{{display:flex;gap:20px;font-size:12px;margin-bottom:8px;flex-wrap:wrap}}
  .legend-item{{display:flex;align-items:center;gap:6px}}
  .dot{{width:12px;height:12px;border-radius:50%;display:inline-block}}
</style>
</head>
<body>
<header>
  <h1>Entra Access Graph Engine</h1>
  <p>Identity Risk Analysis Report</p>
</header>
<main>
  <div class="grid">
    <div class="card"><h3>Total Nodes</h3><div class="num" id="t-nodes">-</div></div>
    <div class="card"><h3>Critical</h3><div class="num critical" id="t-crit">-</div></div>
    <div class="card"><h3>High Risk</h3><div class="num high" id="t-high">-</div></div>
    <div class="card"><h3>Privilege Chains</h3><div class="num" id="t-chains">-</div></div>
  </div>

  <div id="svg-wrap">
    <h2>Access Graph</h2>
    <div class="legend">
      <span class="legend-item"><span class="dot" style="background:#d13438"></span>Critical</span>
      <span class="legend-item"><span class="dot" style="background:#d83b01"></span>High</span>
      <span class="legend-item"><span class="dot" style="background:#ca5010"></span>Medium</span>
      <span class="legend-item"><span class="dot" style="background:#107c10"></span>Low</span>
    </div>
    <svg id="graph-svg"></svg>
  </div>

  <h2 class="section">High Risk Nodes</h2>
  <table><thead><tr><th>Name</th><th>Type</th><th>Risk</th></tr></thead>
  <tbody id="risk-tbody"></tbody></table>

  <h2 class="section">Privilege Escalation Chains</h2>
  <div id="chains"></div>
</main>

<script src="https://d3js.org/d3.v7.min.js"></script>
<script>
const REPORT = {report_json};
const GRAPH = {graph_json};

document.getElementById('t-nodes').textContent  = REPORT.summary.total_nodes;
document.getElementById('t-crit').textContent   = REPORT.summary.critical_nodes;
document.getElementById('t-high').textContent   = REPORT.summary.high_nodes;
document.getElementById('t-chains').textContent = REPORT.summary.total_chains;

const tbody = document.getElementById('risk-tbody');
REPORT.high_risk_nodes.forEach(n => {{
  const tr = document.createElement('tr');
  tr.innerHTML = `<td>${{n.display_name}}</td><td>${{n.node_type}}</td><td><span class="badge badge-${{n.risk_level}}">${{n.risk_level}}</span></td>`;
  tbody.appendChild(tr);
}});

const chainsDiv = document.getElementById('chains');
REPORT.privilege_chains.forEach(c => {{
  const d = document.createElement('div');
  d.className = `chain-card ${{c.risk_level}}`;
  d.innerHTML = `<div class="chain-path">${{c.node_names.join(' &#8594; ')}}</div><div class="chain-desc">${{c.description}}</div>`;
  chainsDiv.appendChild(d);
}});

const colorMap = {{critical:'#d13438',high:'#d83b01',medium:'#ca5010',low:'#107c10'}};
const nodes = Object.values(GRAPH.nodes).map(n => ({{...n}}));
const links = GRAPH.edges.map(e => ({{source:e.from_id,target:e.to_id,type:e.edge_type}}));

const svg = d3.select('#graph-svg');
const w = document.getElementById('graph-svg').clientWidth || 900;
const h = 520;

const sim = d3.forceSimulation(nodes)
  .force('link', d3.forceLink(links).id(d => d.id).distance(120))
  .force('charge', d3.forceManyBody().strength(-400))
  .force('center', d3.forceCenter(w/2, h/2))
  .force('collision', d3.forceCollide(24))
  .force('x', d3.forceX(w/2).strength(0.05))
  .force('y', d3.forceY(h/2).strength(0.05));

svg.append('defs').append('marker')
  .attr('id','arrow').attr('viewBox','0 -5 10 10').attr('refX',20).attr('refY',0)
  .attr('markerWidth',6).attr('markerHeight',6).attr('orient','auto')
  .append('path').attr('d','M0,-5L10,0L0,5').attr('fill','#c8c6c4');

const link = svg.append('g').selectAll('line').data(links).enter().append('line')
  .attr('class','link').attr('stroke-width',1.5).attr('marker-end','url(#arrow)');

const node = svg.append('g').selectAll('g').data(nodes).enter().append('g').attr('class','node')
  .call(d3.drag()
    .on('start',(e,d)=>{{if(!e.active)sim.alphaTarget(.3).restart();d.fx=d.x;d.fy=d.y;}})
    .on('drag',(e,d)=>{{d.fx=e.x;d.fy=e.y;}})
    .on('end',(e,d)=>{{if(!e.active)sim.alphaTarget(0);d.fx=null;d.fy=null;}}));

node.append('circle').attr('r',10).attr('fill',d=>colorMap[d.risk_level]||'#0078d4').attr('stroke','white').attr('stroke-width',1.5);
node.append('title').text(d=>`${{d.display_name}} [${{d.node_type}}] · ${{d.risk_level}}`);
node.append('text').attr('dx',14).attr('dy','.35em').text(d=>d.display_name.length>22?d.display_name.substring(0,21)+'…':d.display_name);

sim.on('tick',()=>{{
  link.attr('x1',d=>d.source.x).attr('y1',d=>d.source.y).attr('x2',d=>d.target.x).attr('y2',d=>d.target.y);
  node.attr('transform',d=>`translate(${{d.x}},${{d.y}})`);
}});
</script>
<footer>Generated by Entra Access Graph Engine · RayStudio</footer>
</body>
</html>"#,
        report_json = report_json,
        graph_json = graph_json
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Node, NodeType, RiskLevel, RiskReport};
    use tempfile::NamedTempFile;

    fn sample_graph() -> AccessGraph {
        let mut g = AccessGraph::default();
        let mut n = Node::new("u1".to_string(), "Alice".to_string(), NodeType::User);
        n.risk_level = RiskLevel::High;
        g.add_node(n);
        g
    }

    #[test]
    fn json_export_roundtrip() {
        let report = RiskReport::default();
        let tmp = NamedTempFile::new().unwrap();
        export_json(&report, tmp.path()).unwrap();
        let content = std::fs::read_to_string(tmp.path()).unwrap();
        let parsed: RiskReport = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.summary.total_nodes, 0);
    }

    #[test]
    fn graphml_export_contains_xml_header() {
        let g = sample_graph();
        let tmp = NamedTempFile::new().unwrap();
        export_graphml(&g, tmp.path()).unwrap();
        let content = std::fs::read_to_string(tmp.path()).unwrap();
        assert!(content.contains("<?xml"));
        assert!(content.contains("<graphml"));
        assert!(content.contains("Alice"));
    }

    #[test]
    fn html_export_contains_d3() {
        let report = RiskReport::default();
        let g = AccessGraph::default();
        let tmp = NamedTempFile::new().unwrap();
        export_html(&report, &g, tmp.path()).unwrap();
        let content = std::fs::read_to_string(tmp.path()).unwrap();
        assert!(content.contains("d3js.org"));
        assert!(content.contains("Entra Access Graph Engine"));
    }

    #[test]
    fn xml_escape_replaces_specials() {
        assert_eq!(xml_escape("a&b<c>d\"e'f"), "a&amp;b&lt;c&gt;d&quot;e&apos;f");
    }
}
