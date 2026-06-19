use std::collections::VecDeque;

use petgraph::Direction;

use crate::{
    edge_analyzer::EntraGraph,
    model::{AccessGraph, PrivilegeChain, RiskLevel},
};

const MAX_DEPTH: usize = 6;

struct SearchCtx<'a> {
    graph: &'a EntraGraph,
    access_graph: &'a AccessGraph,
    min_risk: &'a RiskLevel,
}

pub fn find_privilege_chains(
    graph: &EntraGraph,
    access_graph: &AccessGraph,
    min_risk: &RiskLevel,
) -> Vec<PrivilegeChain> {
    let ctx = SearchCtx { graph, access_graph, min_risk };
    let mut chains = Vec::new();

    // Find target nodes at or above min_risk
    let targets: Vec<_> = access_graph
        .nodes
        .values()
        .filter(|n| &n.risk_level >= min_risk)
        .filter_map(|n| graph.node_index_map.get(&n.id).copied())
        .collect();

    for target in targets {
        // BFS backwards from each high-risk node
        // Each queue entry is a path stored in reverse: [target, pred, ...]
        let mut queue: VecDeque<Vec<_>> = VecDeque::new();
        queue.push_back(vec![target]);

        while let Some(path) = queue.pop_front() {
            let current = *path.last().unwrap();
            let depth = path.len();

            let predecessors: Vec<_> = ctx
                .graph
                .graph
                .neighbors_directed(current, Direction::Incoming)
                .filter(|p| !path.contains(p))
                .collect();

            if predecessors.is_empty() || depth >= MAX_DEPTH {
                if depth >= 2 {
                    let forward: Vec<String> = path
                        .iter()
                        .rev()
                        .map(|idx| ctx.graph.graph[*idx].clone())
                        .collect();
                    if let Some(chain) = build_chain(&forward, &ctx) {
                        chains.push(chain);
                    }
                }
                continue;
            }

            for pred in predecessors {
                let mut new_path = path.clone();
                new_path.push(pred);
                queue.push_back(new_path);
            }
        }
    }

    // Deduplicate by path
    chains.sort_by(|a, b| a.path.cmp(&b.path));
    chains.dedup_by(|a, b| a.path == b.path);
    chains
}

fn build_chain(path: &[String], ctx: &SearchCtx<'_>) -> Option<PrivilegeChain> {
    let node_names: Vec<String> = path
        .iter()
        .filter_map(|id| ctx.access_graph.nodes.get(id))
        .map(|n| n.display_name.clone())
        .collect();

    if node_names.len() < 2 {
        return None;
    }

    let target = ctx.access_graph.nodes.get(path.last()?)?;
    let description = format!(
        "{}-hop chain reaching {} ({})",
        path.len() - 1,
        target.display_name,
        target.risk_level
    );

    Some(PrivilegeChain {
        path: path.to_vec(),
        node_names,
        risk_level: target.risk_level.clone(),
        description,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        edge_analyzer::EntraGraph,
        model::{AccessGraph, EdgeType, Node, NodeType, RiskLevel},
        risk_scorer::score_graph,
    };

    fn test_graph() -> (AccessGraph, EntraGraph) {
        let mut ag = AccessGraph::default();

        ag.add_node(Node::new("alice".to_string(), "Alice".to_string(), NodeType::User));
        ag.add_node(Node::new("admins".to_string(), "IT Admins".to_string(), NodeType::Group));

        let mut role = Node::new(
            "global-admin".to_string(),
            "Global Administrator".to_string(),
            NodeType::DirectoryRole,
        );
        role.properties.insert(
            "roleTemplateId".to_string(),
            serde_json::json!("62e90394-69f5-4237-9190-012177145e10"),
        );
        ag.add_node(role);

        ag.add_edge("alice".to_string(), "admins".to_string(), EdgeType::MemberOf);
        ag.add_edge("admins".to_string(), "global-admin".to_string(), EdgeType::RoleAssignment);

        score_graph(&mut ag);

        let eg = EntraGraph::build_from(&ag);
        (ag, eg)
    }

    #[test]
    fn detects_user_to_global_admin_chain() {
        let (ag, eg) = test_graph();
        let chains = find_privilege_chains(&eg, &ag, &RiskLevel::High);
        assert!(!chains.is_empty(), "Should detect at least one chain");
        let found = chains
            .iter()
            .any(|c| c.path.contains(&"alice".to_string()) && c.path.contains(&"global-admin".to_string()));
        assert!(found, "Chain should include alice and global-admin");
    }

    #[test]
    fn no_chains_below_threshold() {
        let (ag, eg) = test_graph();
        // Filter for Critical only — alice→admins→Global Admin IS Critical
        let chains = find_privilege_chains(&eg, &ag, &RiskLevel::Critical);
        assert!(!chains.is_empty());
    }

    #[test]
    fn no_chains_on_empty_graph() {
        let ag = AccessGraph::default();
        let eg = EntraGraph::build_from(&ag);
        let chains = find_privilege_chains(&eg, &ag, &RiskLevel::High);
        assert!(chains.is_empty());
    }
}
