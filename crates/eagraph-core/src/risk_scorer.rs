use std::collections::HashSet;

use crate::model::{AccessGraph, Node, NodeType, PrivilegeChain, ReportSummary, RiskLevel};

const CRITICAL_ROLE_IDS: &[&str] = &[
    "62e90394-69f5-4237-9190-012177145e10", // Global Administrator
    "e8611ab8-c189-46e8-94e1-60213ab1f814", // Privileged Role Administrator
    "7be44c8a-adaf-4e2a-84d6-ab2649e08a13", // Privileged Authentication Administrator
];

const HIGH_RISK_ROLE_IDS: &[&str] = &[
    "9b895d92-2cd3-44c7-9d02-a6ac2d5ea5c3", // Application Administrator
    "158c047a-c907-4556-b7ef-446551a6b5f7", // Cloud Application Administrator
    "fe930be7-5e62-47db-91af-98c3a49a38b1", // User Administrator
    "29232cdf-9323-42fd-ade2-1d097af3e4de", // Exchange Administrator
    "194ae4cb-b126-40b2-bd5b-6091b380977d", // Security Administrator
    "f28a1f50-f6e7-4571-818b-6a12f2af6b6c", // SharePoint Administrator
    "729827e3-9c14-49f7-bb1b-9608f156bbb8", // Helpdesk Administrator
];

const CRITICAL_PERMISSIONS: &[&str] = &[
    "RoleManagement.ReadWrite.Directory",
    "Directory.ReadWrite.All",
    "AppRoleAssignment.ReadWrite.All",
    "Policy.ReadWrite.ConditionalAccess",
];

const HIGH_RISK_PERMISSIONS: &[&str] = &[
    "User.ReadWrite.All",
    "Group.ReadWrite.All",
    "Application.ReadWrite.All",
    "Directory.Read.All",
    "Mail.ReadWrite",
    "Files.ReadWrite.All",
    "Sites.ReadWrite.All",
];

pub fn score_node(node: &Node) -> RiskLevel {
    match node.node_type {
        NodeType::DirectoryRole => score_directory_role(node),
        NodeType::Application | NodeType::ServicePrincipal => score_app_or_sp(node),
        NodeType::User | NodeType::Group => RiskLevel::Low,
    }
}

fn score_directory_role(node: &Node) -> RiskLevel {
    let role_id = node
        .properties
        .get("roleTemplateId")
        .and_then(|v| v.as_str())
        .unwrap_or(node.id.as_str());

    if CRITICAL_ROLE_IDS.contains(&role_id) {
        return RiskLevel::Critical;
    }
    if HIGH_RISK_ROLE_IDS.contains(&role_id) {
        return RiskLevel::High;
    }
    RiskLevel::Medium
}

fn score_app_or_sp(node: &Node) -> RiskLevel {
    let permissions: HashSet<&str> = node
        .properties
        .get("permissions")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|p| p.as_str()).collect())
        .unwrap_or_default();

    if permissions.iter().any(|p| CRITICAL_PERMISSIONS.contains(p)) {
        return RiskLevel::Critical;
    }
    if permissions.iter().any(|p| HIGH_RISK_PERMISSIONS.contains(p)) {
        return RiskLevel::High;
    }
    if !permissions.is_empty() {
        return RiskLevel::Medium;
    }
    RiskLevel::Low
}

pub fn score_graph(graph: &mut AccessGraph) {
    let scores: Vec<(String, RiskLevel)> = graph
        .nodes
        .values()
        .map(|n| (n.id.clone(), score_node(n)))
        .collect();

    for (id, risk) in scores {
        if let Some(node) = graph.nodes.get_mut(&id) {
            node.risk_level = risk;
        }
    }
}

pub fn build_report_summary(
    graph: &AccessGraph,
    chains: &[PrivilegeChain],
) -> ReportSummary {
    let mut summary = ReportSummary {
        total_nodes: graph.nodes.len(),
        total_edges: graph.edges.len(),
        total_chains: chains.len(),
        ..Default::default()
    };
    for node in graph.nodes.values() {
        match node.risk_level {
            RiskLevel::Critical => summary.critical_nodes += 1,
            RiskLevel::High => summary.high_nodes += 1,
            RiskLevel::Medium => summary.medium_nodes += 1,
            RiskLevel::Low => summary.low_nodes += 1,
        }
    }
    summary
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Node, NodeType};
    use std::collections::HashMap;

    fn role_node(id: &str, template_id: &str) -> Node {
        let mut props = HashMap::new();
        props.insert(
            "roleTemplateId".to_string(),
            serde_json::json!(template_id),
        );
        Node {
            id: id.to_string(),
            display_name: "Test Role".to_string(),
            node_type: NodeType::DirectoryRole,
            risk_level: RiskLevel::Low,
            properties: props,
        }
    }

    fn app_node(permissions: &[&str]) -> Node {
        let mut props = HashMap::new();
        props.insert("permissions".to_string(), serde_json::json!(permissions));
        Node {
            id: "app1".to_string(),
            display_name: "App".to_string(),
            node_type: NodeType::Application,
            risk_level: RiskLevel::Low,
            properties: props,
        }
    }

    #[test]
    fn global_admin_is_critical() {
        let n = role_node("r1", "62e90394-69f5-4237-9190-012177145e10");
        assert_eq!(score_node(&n), RiskLevel::Critical);
    }

    #[test]
    fn app_admin_is_high() {
        let n = role_node("r2", "9b895d92-2cd3-44c7-9d02-a6ac2d5ea5c3");
        assert_eq!(score_node(&n), RiskLevel::High);
    }

    #[test]
    fn unknown_role_is_medium() {
        let n = role_node("r3", "00000000-0000-0000-0000-000000000000");
        assert_eq!(score_node(&n), RiskLevel::Medium);
    }

    #[test]
    fn app_with_critical_permission_is_critical() {
        let n = app_node(&["RoleManagement.ReadWrite.Directory"]);
        assert_eq!(score_node(&n), RiskLevel::Critical);
    }

    #[test]
    fn app_with_high_permission_is_high() {
        let n = app_node(&["User.ReadWrite.All"]);
        assert_eq!(score_node(&n), RiskLevel::High);
    }

    #[test]
    fn app_no_permissions_is_low() {
        let n = app_node(&[]);
        assert_eq!(score_node(&n), RiskLevel::Low);
    }

    #[test]
    fn risk_level_ordering() {
        assert!(RiskLevel::Low < RiskLevel::Medium);
        assert!(RiskLevel::Medium < RiskLevel::High);
        assert!(RiskLevel::High < RiskLevel::Critical);
    }
}
