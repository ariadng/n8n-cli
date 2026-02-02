use crate::error::{N8nError, Result};
use crate::models::common::{format_timestamp_str, truncate};
use crate::models::connection::{Connection, ConnectionsMap};
use crate::models::node::{Node, Position};
use crate::output::Outputable;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Workflow summary (returned by list endpoint)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(default)]
    pub tags: Vec<WorkflowTag>,
}

/// Tag reference in workflow
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowTag {
    pub id: String,
    pub name: String,
}

impl Outputable for Workflow {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "ACTIVE", "TAGS", "UPDATED"]
    }

    fn row(&self) -> Vec<String> {
        let tags = self
            .tags
            .iter()
            .map(|t| t.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        vec![
            self.id.clone(),
            truncate(&self.name, 40),
            if self.active { "yes" } else { "no" }.to_string(),
            truncate(&tags, 20),
            format_timestamp_str(&self.updated_at),
        ]
    }
}

/// Full workflow definition (for create/update)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowDefinition {
    pub name: String,
    #[serde(default)]
    pub nodes: Vec<Value>,
    #[serde(default)]
    pub connections: Value,
    #[serde(default)]
    pub settings: Value,
}

/// Workflow detail response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowDetail {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub nodes: Vec<Value>,
    pub connections: Value,
    #[serde(default)]
    pub settings: Value,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(default)]
    pub tags: Vec<WorkflowTag>,
    #[serde(rename = "versionId", default)]
    pub version_id: Option<String>,
}

/// Common workflow settings
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct WorkflowSettings {
    /// Save execution progress
    #[serde(
        rename = "saveExecutionProgress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub save_execution_progress: Option<bool>,

    /// Save data error execution
    #[serde(
        rename = "saveDataErrorExecution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub save_data_error_execution: Option<String>,

    /// Save data success execution
    #[serde(
        rename = "saveDataSuccessExecution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub save_data_success_execution: Option<String>,

    /// Save manual executions
    #[serde(
        rename = "saveManualExecutions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub save_manual_executions: Option<bool>,

    /// Timezone for the workflow
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    /// Error workflow ID
    #[serde(
        rename = "errorWorkflow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_workflow: Option<String>,

    /// Execution timeout (seconds)
    #[serde(
        rename = "executionTimeout",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_timeout: Option<u32>,

    /// Execution order
    #[serde(
        rename = "executionOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_order: Option<String>,

    /// Catch-all for extra settings
    #[serde(flatten)]
    pub extra: Value,
}

/// Full workflow with typed nodes and connections
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypedWorkflow {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub active: bool,
    pub nodes: Vec<Node>,
    pub connections: ConnectionsMap,
    #[serde(default)]
    pub settings: WorkflowSettings,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<WorkflowTag>,
    #[serde(rename = "versionId", default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl TypedWorkflow {
    /// Convert from WorkflowDetail (API response)
    pub fn from_detail(detail: WorkflowDetail) -> Result<Self> {
        let nodes: Vec<Node> =
            serde_json::from_value(Value::Array(detail.nodes)).map_err(N8nError::InvalidInput)?;

        let connections: ConnectionsMap =
            serde_json::from_value(detail.connections).map_err(N8nError::InvalidInput)?;

        let settings: WorkflowSettings =
            serde_json::from_value(detail.settings).unwrap_or_default();

        Ok(Self {
            id: Some(detail.id),
            name: detail.name,
            active: detail.active,
            nodes,
            connections,
            settings,
            tags: detail.tags,
            version_id: detail.version_id,
        })
    }

    /// Convert to WorkflowDefinition for API updates
    pub fn to_definition(&self) -> WorkflowDefinition {
        WorkflowDefinition {
            name: self.name.clone(),
            nodes: self
                .nodes
                .iter()
                .map(|n| serde_json::to_value(n).unwrap())
                .collect(),
            connections: serde_json::to_value(&self.connections).unwrap(),
            settings: serde_json::to_value(&self.settings).unwrap(),
        }
    }

    /// Find a node by ID or name
    pub fn find_node(&self, node_id: &str) -> Option<&Node> {
        self.nodes
            .iter()
            .find(|n| n.id == node_id || n.name == node_id)
    }

    /// Find a node by ID or name (mutable)
    pub fn find_node_mut(&mut self, node_id: &str) -> Option<&mut Node> {
        self.nodes
            .iter_mut()
            .find(|n| n.id == node_id || n.name == node_id)
    }

    /// Get node name by ID (for connection lookups)
    pub fn get_node_name(&self, node_id: &str) -> Option<String> {
        self.find_node(node_id).map(|n| n.name.clone())
    }

    /// Add a node
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    /// Remove a node and its connections
    pub fn remove_node(&mut self, node_id: &str) -> Option<Node> {
        // Find the node
        let node_name = self.find_node(node_id).map(|n| n.name.clone())?;

        // Remove outgoing connections
        self.connections.remove(&node_name);

        // Remove incoming connections
        for outputs in self.connections.values_mut() {
            for indices in outputs.values_mut() {
                for targets in indices.iter_mut() {
                    targets.retain(|t| t.node != node_name);
                }
            }
        }

        // Remove the node
        let pos = self
            .nodes
            .iter()
            .position(|n| n.id == node_id || n.name == node_id)?;
        Some(self.nodes.remove(pos))
    }

    /// Get flattened connections
    pub fn connections_flat(&self) -> Vec<Connection> {
        Connection::from_connections_map(&self.connections)
    }

    /// Add a connection
    pub fn add_connection(&mut self, conn: Connection) {
        Connection::add_to_map(&mut self.connections, &conn);
    }

    /// Remove a connection
    pub fn remove_connection(&mut self, from_node: &str, to_node: &str) -> bool {
        // Resolve node names
        let from_name = self
            .get_node_name(from_node)
            .unwrap_or_else(|| from_node.to_string());
        let to_name = self
            .get_node_name(to_node)
            .unwrap_or_else(|| to_node.to_string());

        Connection::remove_from_map(&mut self.connections, &from_name, &to_name)
    }

    /// Calculate auto position for a new node
    pub fn auto_position(&self) -> Position {
        let max_x = self.nodes.iter().map(|n| n.position.x).max().unwrap_or(0);
        Position::new(max_x + 200, 100)
    }

    /// Check if workflow has any trigger nodes
    pub fn has_trigger(&self) -> bool {
        self.nodes.iter().any(|n| n.is_trigger())
    }

    /// Get all node names (for connection validation)
    pub fn node_names(&self) -> Vec<&str> {
        self.nodes.iter().map(|n| n.name.as_str()).collect()
    }

    /// Rename a node in all connection references
    pub fn rename_node_in_connections(&mut self, old_name: &str, new_name: &str) {
        // Update source keys (outgoing connections)
        if let Some(outputs) = self.connections.remove(old_name) {
            self.connections.insert(new_name.to_string(), outputs);
        }

        // Update target references (incoming connections)
        for outputs in self.connections.values_mut() {
            for indices in outputs.values_mut() {
                for targets in indices.iter_mut() {
                    for target in targets.iter_mut() {
                        if target.node == old_name {
                            target.node = new_name.to_string();
                        }
                    }
                }
            }
        }
    }
}
