use crate::models::common::truncate;
use crate::output::Outputable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single connection endpoint (target of a connection)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ConnectionEndpoint {
    pub node: String,

    #[serde(rename = "type", default = "default_connection_type")]
    pub connection_type: String,

    #[serde(default)]
    pub index: u32,
}

fn default_connection_type() -> String {
    "main".to_string()
}

/// n8n's native connection format (source-keyed)
/// Format: { "NodeName": { "main": [[{ "node": "TargetNode", "type": "main", "index": 0 }]] } }
pub type ConnectionsMap = HashMap<String, HashMap<String, Vec<Vec<ConnectionEndpoint>>>>;

/// Flattened connection for display/manipulation
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Connection {
    pub source_node: String,
    pub source_output: u32,
    pub source_type: String,
    pub target_node: String,
    pub target_input: u32,
    pub target_type: String,
}

impl Connection {
    /// Create a new connection with default types
    pub fn new(source_node: String, target_node: String) -> Self {
        Self {
            source_node,
            source_output: 0,
            source_type: "main".to_string(),
            target_node,
            target_input: 0,
            target_type: "main".to_string(),
        }
    }

    /// Create a connection with full parameters
    pub fn new_full(
        source_node: String,
        source_output: u32,
        source_type: String,
        target_node: String,
        target_input: u32,
        target_type: String,
    ) -> Self {
        Self {
            source_node,
            source_output,
            source_type,
            target_node,
            target_input,
            target_type,
        }
    }

    /// Convert from n8n's nested format to flat list
    pub fn from_connections_map(map: &ConnectionsMap) -> Vec<Self> {
        let mut connections = Vec::new();

        for (source_node, outputs) in map {
            for (output_type, output_indices) in outputs {
                for (output_index, targets) in output_indices.iter().enumerate() {
                    for target in targets {
                        connections.push(Connection {
                            source_node: source_node.clone(),
                            source_output: output_index as u32,
                            source_type: output_type.clone(),
                            target_node: target.node.clone(),
                            target_input: target.index,
                            target_type: target.connection_type.clone(),
                        });
                    }
                }
            }
        }

        connections
    }

    /// Convert flat list back to n8n's nested format
    pub fn to_connections_map(connections: &[Self]) -> ConnectionsMap {
        let mut map: ConnectionsMap = HashMap::new();

        for conn in connections {
            let outputs = map.entry(conn.source_node.clone()).or_default();

            let indices = outputs.entry(conn.source_type.clone()).or_default();

            // Ensure we have enough output index slots
            while indices.len() <= conn.source_output as usize {
                indices.push(Vec::new());
            }

            indices[conn.source_output as usize].push(ConnectionEndpoint {
                node: conn.target_node.clone(),
                connection_type: conn.target_type.clone(),
                index: conn.target_input,
            });
        }

        map
    }

    /// Add a single connection to an existing map
    pub fn add_to_map(map: &mut ConnectionsMap, conn: &Connection) {
        let outputs = map.entry(conn.source_node.clone()).or_default();
        let indices = outputs.entry(conn.source_type.clone()).or_default();

        while indices.len() <= conn.source_output as usize {
            indices.push(Vec::new());
        }

        indices[conn.source_output as usize].push(ConnectionEndpoint {
            node: conn.target_node.clone(),
            connection_type: conn.target_type.clone(),
            index: conn.target_input,
        });
    }

    /// Remove connections from source to target in a map
    pub fn remove_from_map(map: &mut ConnectionsMap, from_node: &str, to_node: &str) -> bool {
        let mut removed = false;
        if let Some(outputs) = map.get_mut(from_node) {
            for indices in outputs.values_mut() {
                for targets in indices.iter_mut() {
                    let before = targets.len();
                    targets.retain(|t| t.node != to_node);
                    if targets.len() < before {
                        removed = true;
                    }
                }
            }
        }
        removed
    }
}

impl Outputable for Connection {
    fn headers() -> Vec<&'static str> {
        vec!["FROM NODE", "OUTPUT", "TO NODE", "INPUT"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            truncate(&self.source_node, 25),
            format!("{}[{}]", self.source_type, self.source_output),
            truncate(&self.target_node, 25),
            format!("{}[{}]", self.target_type, self.target_input),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connections_roundtrip() {
        let connections = vec![
            Connection::new("Node1".to_string(), "Node2".to_string()),
            Connection::new_full(
                "Node2".to_string(),
                0,
                "main".to_string(),
                "Node3".to_string(),
                0,
                "main".to_string(),
            ),
        ];

        let map = Connection::to_connections_map(&connections);
        let flat = Connection::from_connections_map(&map);

        assert_eq!(connections.len(), flat.len());
    }

    #[test]
    fn test_add_remove_connection() {
        let mut map: ConnectionsMap = HashMap::new();

        let conn = Connection::new("A".to_string(), "B".to_string());
        Connection::add_to_map(&mut map, &conn);

        let flat = Connection::from_connections_map(&map);
        assert_eq!(flat.len(), 1);

        let removed = Connection::remove_from_map(&mut map, "A", "B");
        assert!(removed);

        let flat = Connection::from_connections_map(&map);
        assert_eq!(flat.len(), 0);
    }
}
