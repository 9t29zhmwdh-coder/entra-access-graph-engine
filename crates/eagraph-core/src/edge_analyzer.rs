use std::collections::HashMap;

use petgraph::{
    graph::{DiGraph, NodeIndex},
    Direction,
};

use crate::model::{AccessGraph, EdgeType};

pub struct EntraGraph {
    pub graph: DiGraph<String, EdgeType>,
    pub node_index_map: HashMap<String, NodeIndex>,
}

impl EntraGraph {
    pub fn build_from(access_graph: &AccessGraph) -> Self {
        let mut graph = DiGraph::new();
        let mut node_index_map = HashMap::new();

        for id in access_graph.nodes.keys() {
            let idx = graph.add_node(id.clone());
            node_index_map.insert(id.clone(), idx);
        }

        for edge in &access_graph.edges {
            if let (Some(&from_idx), Some(&to_idx)) = (
                node_index_map.get(&edge.from_id),
                node_index_map.get(&edge.to_id),
            ) {
                graph.add_edge(from_idx, to_idx, edge.edge_type.clone());
            }
        }

        Self { graph, node_index_map }
    }

    pub fn predecessors(&self, id: &str) -> Vec<String> {
        match self.node_index_map.get(id) {
            Some(&idx) => self
                .graph
                .neighbors_directed(idx, Direction::Incoming)
                .map(|pred| self.graph[pred].clone())
                .collect(),
            None => vec![],
        }
    }

    pub fn successors(&self, id: &str) -> Vec<String> {
        match self.node_index_map.get(id) {
            Some(&idx) => self
                .graph
                .neighbors_directed(idx, Direction::Outgoing)
                .map(|succ| self.graph[succ].clone())
                .collect(),
            None => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Node, NodeType};

    #[test]
    fn builds_graph_from_access_graph() {
        let mut ag = AccessGraph::default();
        ag.add_node(Node::new("u1".to_string(), "User".to_string(), NodeType::User));
        ag.add_node(Node::new("g1".to_string(), "Group".to_string(), NodeType::Group));
        ag.add_edge("u1".to_string(), "g1".to_string(), EdgeType::MemberOf);

        let eg = EntraGraph::build_from(&ag);
        assert_eq!(eg.graph.node_count(), 2);
        assert_eq!(eg.graph.edge_count(), 1);
    }

    #[test]
    fn predecessors_finds_incoming_nodes() {
        let mut ag = AccessGraph::default();
        ag.add_node(Node::new("u1".to_string(), "User".to_string(), NodeType::User));
        ag.add_node(Node::new("g1".to_string(), "Group".to_string(), NodeType::Group));
        ag.add_edge("u1".to_string(), "g1".to_string(), EdgeType::MemberOf);

        let eg = EntraGraph::build_from(&ag);
        let preds = eg.predecessors("g1");
        assert_eq!(preds, vec!["u1"]);
    }
}
