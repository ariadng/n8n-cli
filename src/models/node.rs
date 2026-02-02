use crate::models::common::truncate;
use crate::output::Outputable;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// Position of a node on the canvas (serializes as [x, y] array)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as [x, y] array
        [self.x, self.y].serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize from [x, y] array
        let arr: [i32; 2] = Deserialize::deserialize(deserializer)?;
        Ok(Position { x: arr[0], y: arr[1] })
    }
}

/// A workflow node with typed fields
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node {
    /// Unique node identifier within the workflow
    pub id: String,

    /// Display name shown in the UI
    pub name: String,

    /// n8n node type (e.g., "n8n-nodes-base.httpRequest")
    #[serde(rename = "type")]
    pub node_type: String,

    /// Version of the node type
    #[serde(rename = "typeVersion", default = "default_type_version")]
    pub type_version: f64,

    /// Canvas position
    #[serde(default)]
    pub position: Position,

    /// Node-specific parameters (type depends on node_type)
    #[serde(default)]
    pub parameters: Value,

    /// Credential references
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Value>,

    /// Whether the node is disabled
    #[serde(default)]
    pub disabled: bool,

    /// Notes/comments on the node
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Continue on fail setting
    #[serde(rename = "continueOnFail", default)]
    pub continue_on_fail: bool,

    /// Retry on fail settings
    #[serde(rename = "retryOnFail", default)]
    pub retry_on_fail: bool,

    /// Maximum retries
    #[serde(rename = "maxTries", default, skip_serializing_if = "Option::is_none")]
    pub max_tries: Option<u32>,

    /// Wait between retries (ms)
    #[serde(
        rename = "waitBetweenTries",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wait_between_tries: Option<u32>,

    /// Always output data
    #[serde(rename = "alwaysOutputData", default)]
    pub always_output_data: bool,

    /// Execute once setting
    #[serde(rename = "executeOnce", default)]
    pub execute_once: bool,

    /// Webhook ID (for webhook nodes)
    #[serde(rename = "webhookId", default, skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,

    /// Catch-all for unknown fields
    #[serde(flatten)]
    pub extra: Value,
}

fn default_type_version() -> f64 {
    1.0
}

impl Node {
    /// Create a new node with minimal required fields
    pub fn new(id: String, name: String, node_type: String) -> Self {
        Self {
            id,
            name,
            node_type,
            type_version: 1.0,
            position: Position::default(),
            parameters: Value::Object(serde_json::Map::new()),
            credentials: None,
            disabled: false,
            notes: None,
            continue_on_fail: false,
            retry_on_fail: false,
            max_tries: None,
            wait_between_tries: None,
            always_output_data: false,
            execute_once: false,
            webhook_id: None,
            extra: Value::Object(serde_json::Map::new()),
        }
    }

    /// Generate a unique node ID
    pub fn generate_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Check if this is a trigger node
    pub fn is_trigger(&self) -> bool {
        self.node_type.contains("trigger")
            || self.node_type.contains("Trigger")
            || self.node_type == "n8n-nodes-base.start"
            || self.node_type == "n8n-nodes-base.manualTrigger"
    }

    /// Set position
    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.position = Position::new(x, y);
        self
    }

    /// Set parameters
    pub fn with_parameters(mut self, parameters: Value) -> Self {
        self.parameters = parameters;
        self
    }

    /// Set disabled
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Outputable for Node {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "NAME", "TYPE", "POSITION", "DISABLED"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            truncate(&self.id, 20),
            truncate(&self.name, 30),
            truncate(&self.node_type, 35),
            format!("{},{}", self.position.x, self.position.y),
            if self.disabled { "yes" } else { "no" }.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(
            "test-id".to_string(),
            "Test Node".to_string(),
            "n8n-nodes-base.httpRequest".to_string(),
        );
        assert_eq!(node.name, "Test Node");
        assert_eq!(node.type_version, 1.0);
        assert!(!node.disabled);
    }

    #[test]
    fn test_node_serialization_roundtrip() {
        let node = Node::new(
            "test-id".to_string(),
            "Test Node".to_string(),
            "n8n-nodes-base.httpRequest".to_string(),
        );
        let json = serde_json::to_string(&node).unwrap();
        let parsed: Node = serde_json::from_str(&json).unwrap();
        assert_eq!(node.id, parsed.id);
        assert_eq!(node.name, parsed.name);
    }

    #[test]
    fn test_is_trigger() {
        let trigger = Node::new(
            "1".to_string(),
            "Trigger".to_string(),
            "n8n-nodes-base.manualTrigger".to_string(),
        );
        assert!(trigger.is_trigger());

        let non_trigger = Node::new(
            "2".to_string(),
            "HTTP".to_string(),
            "n8n-nodes-base.httpRequest".to_string(),
        );
        assert!(!non_trigger.is_trigger());
    }
}
