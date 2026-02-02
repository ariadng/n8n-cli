use crate::models::TypedWorkflow;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValidationSeverity {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub message: String,
    pub node: Option<String>,
}

pub struct ValidationResult {
    pub issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|i| matches!(i.severity, ValidationSeverity::Error))
    }

    pub fn errors(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| matches!(i.severity, ValidationSeverity::Error))
            .collect()
    }

    pub fn warnings(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|i| matches!(i.severity, ValidationSeverity::Warning))
            .collect()
    }

    /// Format issues as a string for display
    pub fn format(&self, include_warnings: bool) -> String {
        let mut output = Vec::new();

        for issue in &self.issues {
            if !include_warnings && matches!(issue.severity, ValidationSeverity::Warning) {
                continue;
            }

            let prefix = match issue.severity {
                ValidationSeverity::Error => "ERROR",
                ValidationSeverity::Warning => "WARNING",
            };

            let node_info = issue
                .node
                .as_ref()
                .map(|n| format!(" [{}]", n))
                .unwrap_or_default();

            output.push(format!("{}{}: {}", prefix, node_info, issue.message));
        }

        output.join("\n")
    }
}

/// Validate a workflow
pub fn validate_workflow(workflow: &TypedWorkflow) -> ValidationResult {
    let mut issues = Vec::new();

    // Check for empty workflow
    if workflow.nodes.is_empty() {
        issues.push(ValidationIssue {
            severity: ValidationSeverity::Warning,
            message: "Workflow has no nodes".to_string(),
            node: None,
        });
        return ValidationResult { issues };
    }

    // Check for duplicate node IDs
    let mut seen_ids = HashSet::new();
    for node in &workflow.nodes {
        if !seen_ids.insert(&node.id) {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                message: format!("Duplicate node ID: {}", node.id),
                node: Some(node.name.clone()),
            });
        }
    }

    // Check for duplicate node names (n8n uses names in connections)
    let mut seen_names = HashSet::new();
    for node in &workflow.nodes {
        if !seen_names.insert(&node.name) {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                message: format!("Duplicate node name: {}", node.name),
                node: Some(node.name.clone()),
            });
        }
    }

    // Build set of valid node names
    let valid_nodes: HashSet<_> = workflow.nodes.iter().map(|n| n.name.as_str()).collect();

    // Check for trigger nodes
    let has_trigger = workflow.has_trigger();

    if !has_trigger {
        issues.push(ValidationIssue {
            severity: ValidationSeverity::Warning,
            message: "No trigger node found. Workflow can only be executed manually.".to_string(),
            node: None,
        });
    }

    // Check connections reference valid nodes
    let connections = workflow.connections_flat();
    for conn in &connections {
        if !valid_nodes.contains(conn.source_node.as_str()) {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                message: format!(
                    "Connection references non-existent source node: {}",
                    conn.source_node
                ),
                node: Some(conn.source_node.clone()),
            });
        }
        if !valid_nodes.contains(conn.target_node.as_str()) {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                message: format!(
                    "Connection references non-existent target node: {}",
                    conn.target_node
                ),
                node: Some(conn.target_node.clone()),
            });
        }
    }

    // Check for orphan nodes (not connected to anything except triggers)
    let mut connected_nodes: HashSet<&str> = HashSet::new();
    for conn in &connections {
        connected_nodes.insert(&conn.source_node);
        connected_nodes.insert(&conn.target_node);
    }

    for node in &workflow.nodes {
        let is_trigger = node.is_trigger();

        if !is_trigger && !connected_nodes.contains(node.name.as_str()) {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                message: format!("Node '{}' is not connected to any other node", node.name),
                node: Some(node.name.clone()),
            });
        }
    }

    // Check for self-loops
    for conn in &connections {
        if conn.source_node == conn.target_node {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                message: format!("Node '{}' has a self-loop connection", conn.source_node),
                node: Some(conn.source_node.clone()),
            });
        }
    }

    // Check for empty node names
    for node in &workflow.nodes {
        if node.name.trim().is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                message: "Node has empty name".to_string(),
                node: Some(node.id.clone()),
            });
        }
    }

    // Check for empty workflow name
    if workflow.name.trim().is_empty() {
        issues.push(ValidationIssue {
            severity: ValidationSeverity::Error,
            message: "Workflow has empty name".to_string(),
            node: None,
        });
    }

    ValidationResult { issues }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Node;
    use std::collections::HashMap;

    #[test]
    fn test_validate_empty_workflow() {
        let workflow = TypedWorkflow {
            id: None,
            name: "Test".to_string(),
            active: false,
            nodes: vec![],
            connections: HashMap::new(),
            settings: Default::default(),
            tags: vec![],
            version_id: None,
        };

        let result = validate_workflow(&workflow);
        assert!(result.warnings().len() > 0);
    }

    #[test]
    fn test_validate_duplicate_names() {
        let workflow = TypedWorkflow {
            id: None,
            name: "Test".to_string(),
            active: false,
            nodes: vec![
                Node::new("1".into(), "Same".into(), "type".into()),
                Node::new("2".into(), "Same".into(), "type".into()),
            ],
            connections: HashMap::new(),
            settings: Default::default(),
            tags: vec![],
            version_id: None,
        };

        let result = validate_workflow(&workflow);
        assert!(!result.is_valid());
    }
}
