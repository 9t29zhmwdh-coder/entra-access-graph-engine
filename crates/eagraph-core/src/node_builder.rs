use anyhow::Result;
use serde_json::Value;
use tracing::{info, warn};

use crate::{
    graph_client::GraphClient,
    model::{AccessGraph, EdgeType, Node, NodeType},
};

pub async fn build_access_graph(client: &GraphClient) -> Result<AccessGraph> {
    let token = client.authenticate().await?;
    let mut graph = AccessGraph::default();

    info!("Fetching users...");
    for u in client.get_users(&token).await? {
        if let Some(node) = parse_user(&u) {
            graph.add_node(node);
        }
    }
    info!("Loaded {} users", graph.nodes.len());

    info!("Fetching groups...");
    let groups = client.get_groups(&token).await?;
    for g in &groups {
        if let Some(node) = parse_group(g) {
            graph.add_node(node);
        }
    }

    info!("Fetching group memberships...");
    for g in &groups {
        let group_id = match g["id"].as_str() {
            Some(id) => id.to_string(),
            None => continue,
        };
        match client.get_group_members(&token, &group_id).await {
            Ok(members) => {
                for m in members {
                    if let Some(member_id) = m["id"].as_str() {
                        graph.add_edge(member_id.to_string(), group_id.clone(), EdgeType::MemberOf);
                    }
                }
            }
            Err(e) => warn!("Failed to get members of group {group_id}: {e}"),
        }
    }

    info!("Fetching directory roles...");
    let roles = client.get_directory_roles(&token).await?;
    for r in &roles {
        if let Some(node) = parse_directory_role(r) {
            graph.add_node(node);
        }
    }

    info!("Fetching role assignments...");
    for r in &roles {
        let role_id = match r["id"].as_str() {
            Some(id) => id.to_string(),
            None => continue,
        };
        match client.get_directory_role_members(&token, &role_id).await {
            Ok(members) => {
                for m in members {
                    if let Some(member_id) = m["id"].as_str() {
                        graph.add_edge(
                            member_id.to_string(),
                            role_id.clone(),
                            EdgeType::RoleAssignment,
                        );
                    }
                }
            }
            Err(e) => warn!("Failed to get members of role {role_id}: {e}"),
        }
    }

    info!("Fetching applications...");
    for app in client.get_applications(&token).await? {
        if let Some(node) = parse_application(&app) {
            graph.add_node(node);
        }
    }

    info!("Fetching service principals...");
    let sps = client.get_service_principals(&token).await?;
    for sp in &sps {
        if let Some(node) = parse_service_principal(sp) {
            let sp_id = node.id.clone();
            let app_id = sp["appId"].as_str().unwrap_or("").to_string();
            graph.add_node(node);
            if !app_id.is_empty() {
                if let Some(app_node) = graph.nodes.values().find(|n| {
                    n.properties
                        .get("appId")
                        .and_then(|v| v.as_str())
                        .is_some_and(|id| id == app_id)
                }) {
                    let app_node_id = app_node.id.clone();
                    graph.add_edge(sp_id.clone(), app_node_id, EdgeType::ServicePrincipalOf);
                }
            }
        }
    }

    info!("Fetching app role assignments...");
    for sp in &sps {
        let sp_id = match sp["id"].as_str() {
            Some(id) => id.to_string(),
            None => continue,
        };
        match client.get_sp_app_role_assignments(&token, &sp_id).await {
            Ok(assignments) => {
                for a in assignments {
                    if let Some(resource_id) = a["resourceId"].as_str() {
                        graph.add_edge(
                            sp_id.clone(),
                            resource_id.to_string(),
                            EdgeType::AppRoleAssignment,
                        );
                    }
                }
            }
            Err(e) => warn!("Failed to get app role assignments for SP {sp_id}: {e}"),
        }
    }

    info!("Fetching OAuth2 permission grants...");
    for grant in client.get_oauth2_permission_grants(&token).await? {
        if let (Some(client_id), Some(resource_id)) = (
            grant["clientId"].as_str(),
            grant["resourceId"].as_str(),
        ) {
            graph.add_edge(
                client_id.to_string(),
                resource_id.to_string(),
                EdgeType::OAuthPermissionGrant,
            );
        }
    }

    info!(
        "Graph built: {} nodes, {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
    Ok(graph)
}

pub fn mock_access_graph() -> AccessGraph {
    let mut graph = AccessGraph::default();

    graph.add_node(Node::new(
        "user-alice".to_string(),
        "Alice Meier".to_string(),
        NodeType::User,
    ));
    graph.add_node(Node::new(
        "user-bob".to_string(),
        "Bob Keller".to_string(),
        NodeType::User,
    ));
    graph.add_node(Node::new(
        "group-it-admins".to_string(),
        "IT Admins".to_string(),
        NodeType::Group,
    ));
    graph.add_node(Node::new(
        "app-devops".to_string(),
        "DevOps Pipeline App".to_string(),
        NodeType::Application,
    ));
    graph.add_node(Node::new(
        "sp-devops".to_string(),
        "DevOps Pipeline SP".to_string(),
        NodeType::ServicePrincipal,
    ));

    let mut global_admin = Node::new(
        "role-global-admin".to_string(),
        "Global Administrator".to_string(),
        NodeType::DirectoryRole,
    );
    global_admin
        .properties
        .insert("roleTemplateId".to_string(), serde_json::json!("62e90394-69f5-4237-9190-012177145e10"));
    graph.add_node(global_admin);

    let mut app_admin = Node::new(
        "role-app-admin".to_string(),
        "Application Administrator".to_string(),
        NodeType::DirectoryRole,
    );
    app_admin
        .properties
        .insert("roleTemplateId".to_string(), serde_json::json!("9b895d92-2cd3-44c7-9d02-a6ac2d5ea5c3"));
    graph.add_node(app_admin);

    graph.add_edge("user-alice".to_string(), "group-it-admins".to_string(), EdgeType::MemberOf);
    graph.add_edge("user-bob".to_string(), "group-it-admins".to_string(), EdgeType::MemberOf);
    graph.add_edge("group-it-admins".to_string(), "role-global-admin".to_string(), EdgeType::RoleAssignment);
    graph.add_edge("sp-devops".to_string(), "app-devops".to_string(), EdgeType::ServicePrincipalOf);
    graph.add_edge("sp-devops".to_string(), "role-app-admin".to_string(), EdgeType::AppRoleAssignment);

    graph
}

fn parse_user(v: &Value) -> Option<Node> {
    let id = v["id"].as_str()?.to_string();
    let name = v["displayName"].as_str().unwrap_or("Unknown User").to_string();
    let mut node = Node::new(id, name, NodeType::User);
    if let Some(upn) = v["userPrincipalName"].as_str() {
        node.properties.insert("userPrincipalName".to_string(), serde_json::json!(upn));
    }
    Some(node)
}

fn parse_group(v: &Value) -> Option<Node> {
    let id = v["id"].as_str()?.to_string();
    let name = v["displayName"].as_str().unwrap_or("Unknown Group").to_string();
    Some(Node::new(id, name, NodeType::Group))
}

fn parse_directory_role(v: &Value) -> Option<Node> {
    let id = v["id"].as_str()?.to_string();
    let name = v["displayName"].as_str().unwrap_or("Unknown Role").to_string();
    let mut node = Node::new(id, name, NodeType::DirectoryRole);
    if let Some(template_id) = v["roleTemplateId"].as_str() {
        node.properties.insert("roleTemplateId".to_string(), serde_json::json!(template_id));
    }
    Some(node)
}

fn parse_application(v: &Value) -> Option<Node> {
    let id = v["id"].as_str()?.to_string();
    let name = v["displayName"].as_str().unwrap_or("Unknown App").to_string();
    let mut node = Node::new(id, name, NodeType::Application);
    if let Some(app_id) = v["appId"].as_str() {
        node.properties.insert("appId".to_string(), serde_json::json!(app_id));
    }
    if let Some(resources) = v["requiredResourceAccess"].as_array() {
        let perms: Vec<String> = resources
            .iter()
            .filter_map(|r| r["resourceAccess"].as_array())
            .flatten()
            .filter_map(|a| a["id"].as_str().map(str::to_string))
            .collect();
        node.properties.insert("permissions".to_string(), serde_json::json!(perms));
    }
    Some(node)
}

fn parse_service_principal(v: &Value) -> Option<Node> {
    let id = v["id"].as_str()?.to_string();
    let name = v["displayName"].as_str().unwrap_or("Unknown SP").to_string();
    let mut node = Node::new(id, name, NodeType::ServicePrincipal);
    if let Some(app_id) = v["appId"].as_str() {
        node.properties.insert("appId".to_string(), serde_json::json!(app_id));
    }
    Some(node)
}
