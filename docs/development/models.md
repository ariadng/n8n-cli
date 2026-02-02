# Models

Reference for data structures used throughout the n8n CLI.

## Overview

Models are defined in `src/models/` and serve two purposes:

1. **API Serialization** - Match n8n API request/response structures
2. **Output Formatting** - Implement `Outputable` for table display

## Workflow Models

### Workflow (Summary)

Used in list responses:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

impl Outputable for Workflow {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "Name", "Active", "Updated"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.active.to_string(),
            self.updated_at.clone(),
        ]
    }
}
```

### WorkflowDetail

Full workflow with nodes and connections:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDetail {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub nodes: Value,           // Raw JSON array
    pub connections: Value,     // Raw JSON object
    #[serde(default)]
    pub settings: Value,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
```

### TypedWorkflow

Parsed workflow with typed nodes:

```rust
#[derive(Debug, Clone)]
pub struct TypedWorkflow {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub nodes: Vec<Node>,
    pub connections: ConnectionsMap,
    pub settings: Value,
}

impl TypedWorkflow {
    pub fn from_detail(detail: WorkflowDetail) -> Result<Self> {
        let nodes: Vec<Node> = serde_json::from_value(detail.nodes)?;
        let connections: ConnectionsMap = serde_json::from_value(detail.connections)?;

        Ok(Self {
            id: detail.id,
            name: detail.name,
            active: detail.active,
            nodes,
            connections,
            settings: detail.settings,
        })
    }

    pub fn to_definition(&self) -> WorkflowDefinition {
        WorkflowDefinition {
            name: self.name.clone(),
            nodes: serde_json::to_value(&self.nodes).unwrap(),
            connections: serde_json::to_value(&self.connections).unwrap(),
            settings: Some(self.settings.clone()),
        }
    }
}
```

### WorkflowDefinition

For create/update requests:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub name: String,
    pub nodes: Value,
    pub connections: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Value>,
}
```

## Node Model

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub position: Position,
    #[serde(default)]
    pub parameters: Value,
    #[serde(rename = "typeVersion", default = "default_type_version")]
    pub type_version: u32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Value>,
    #[serde(rename = "webhookId", skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
}

impl Node {
    pub fn new(name: &str, node_type: &str) -> Self {
        Self {
            id: Self::generate_id(),
            name: name.to_string(),
            node_type: node_type.to_string(),
            position: Position::default(),
            parameters: Value::Object(Default::default()),
            type_version: 1,
            disabled: false,
            credentials: None,
            webhook_id: None,
        }
    }

    pub fn generate_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    pub fn is_trigger(&self) -> bool {
        self.node_type.contains("Trigger") || self.node_type.contains("trigger")
    }

    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.position = Position { x, y };
        self
    }

    pub fn with_parameters(mut self, params: Value) -> Self {
        self.parameters = params;
        self
    }
}

impl Outputable for Node {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "Name", "Type", "Position", "Disabled"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.node_type.clone(),
            format!("{},{}", self.position.x, self.position.y),
            self.disabled.to_string(),
        ]
    }
}
```

### Position

Custom serialization for `[x, y]` array format:

```rust
#[derive(Debug, Clone, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [self.x, self.y].serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: [i32; 2] = Deserialize::deserialize(deserializer)?;
        Ok(Position { x: arr[0], y: arr[1] })
    }
}
```

## Connection Models

### Connection (Flattened)

User-friendly format for CLI display:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub source_node: String,
    pub source_output: u32,
    pub source_type: String,
    pub target_node: String,
    pub target_input: u32,
    pub target_type: String,
}

impl Outputable for Connection {
    fn headers() -> Vec<&'static str> {
        vec!["From", "Output", "To", "Input", "Type"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.source_node.clone(),
            self.source_output.to_string(),
            self.target_node.clone(),
            self.target_input.to_string(),
            self.source_type.clone(),
        ]
    }
}
```

### ConnectionsMap

n8n's native nested format:

```rust
pub type ConnectionsMap = HashMap<String, HashMap<String, Vec<Vec<ConnectionEndpoint>>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionEndpoint {
    pub node: String,
    #[serde(rename = "type")]
    pub connection_type: String,
    pub index: u32,
}
```

### Conversion Methods

```rust
impl Connection {
    /// Convert from n8n's nested format to flat list
    pub fn from_connections_map(map: &ConnectionsMap) -> Vec<Self> {
        let mut connections = Vec::new();

        for (source_node, type_map) in map {
            for (source_type, outputs) in type_map {
                for (output_idx, targets) in outputs.iter().enumerate() {
                    for endpoint in targets {
                        connections.push(Connection {
                            source_node: source_node.clone(),
                            source_output: output_idx as u32,
                            source_type: source_type.clone(),
                            target_node: endpoint.node.clone(),
                            target_input: endpoint.index,
                            target_type: endpoint.connection_type.clone(),
                        });
                    }
                }
            }
        }

        connections
    }

    /// Add a connection to the map
    pub fn add_to_map(map: &mut ConnectionsMap, conn: &Connection) {
        let type_map = map
            .entry(conn.source_node.clone())
            .or_insert_with(HashMap::new);

        let outputs = type_map
            .entry(conn.source_type.clone())
            .or_insert_with(Vec::new);

        // Ensure output index exists
        while outputs.len() <= conn.source_output as usize {
            outputs.push(Vec::new());
        }

        outputs[conn.source_output as usize].push(ConnectionEndpoint {
            node: conn.target_node.clone(),
            connection_type: conn.target_type.clone(),
            index: conn.target_input,
        });
    }

    /// Remove a connection from the map
    pub fn remove_from_map(map: &mut ConnectionsMap, source: &str, target: &str) -> bool {
        // ... implementation
    }
}
```

## Execution Models

### Execution (Summary)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    pub id: String,
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    pub status: String,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "stoppedAt")]
    pub stopped_at: Option<String>,
    pub finished: bool,
    pub mode: String,
}

impl Outputable for Execution {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "Workflow", "Status", "Mode", "Started", "Finished"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.workflow_id.clone(),
            self.status.clone(),
            self.mode.clone(),
            self.started_at.clone(),
            self.finished.to_string(),
        ]
    }
}
```

### ExecutionDetail

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionDetail {
    pub id: String,
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    pub status: String,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "stoppedAt")]
    pub stopped_at: Option<String>,
    pub finished: bool,
    pub mode: String,
    #[serde(default)]
    pub data: Option<Value>,
}
```

### ExecuteRequest

```rust
#[derive(Debug, Clone, Serialize)]
pub struct ExecuteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}
```

## Credential Models

### Credential

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub credential_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl Outputable for Credential {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "Name", "Type", "Updated"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.credential_type.clone(),
            self.updated_at.clone(),
        ]
    }
}
```

### CredentialCreate

```rust
#[derive(Debug, Clone, Serialize)]
pub struct CredentialCreate {
    pub name: String,
    #[serde(rename = "type")]
    pub credential_type: String,
    pub data: Value,
}
```

## Tag Models

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl Outputable for Tag {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "Name", "Created", "Updated"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.created_at.clone(),
            self.updated_at.clone(),
        ]
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TagCreate {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TagUpdate {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TagAssignment {
    #[serde(rename = "tagIds")]
    pub tag_ids: Vec<String>,
}
```

## Outputable Trait

```rust
/// Trait for types that can be formatted as table rows
pub trait Outputable {
    /// Column headers for table output
    fn headers() -> Vec<&'static str>;

    /// Row values for table output
    fn row(&self) -> Vec<String>;
}
```

### Implementation Example

```rust
impl Outputable for MyModel {
    fn headers() -> Vec<&'static str> {
        vec!["Column1", "Column2", "Column3"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.field1.clone(),
            self.field2.to_string(),
            self.field3.as_ref().map(|s| s.clone()).unwrap_or_default(),
        ]
    }
}
```

## Serde Patterns

### Rename Fields

```rust
#[serde(rename = "createdAt")]
pub created_at: String,
```

### Skip Serializing None

```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub optional_field: Option<String>,
```

### Default Values

```rust
#[serde(default)]
pub tags: Vec<Tag>,

#[serde(default = "default_type_version")]
pub type_version: u32,

fn default_type_version() -> u32 { 1 }
```

### Custom Serialization

```rust
#[serde(serialize_with = "serialize_position")]
#[serde(deserialize_with = "deserialize_position")]
pub position: Position,
```

---

## See Also

- [API Client](./api-client.md) - How models are used
- [Adding Commands](./adding-commands.md) - Creating new models
- [Code Structure](./code-structure.md) - File organization
