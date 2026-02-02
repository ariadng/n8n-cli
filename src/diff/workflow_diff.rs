use crate::models::{Connection, Node, TypedWorkflow};
use similar::{ChangeTag, TextDiff};
use std::collections::{HashMap, HashSet};

/// Differences between two workflows
#[derive(Debug, Default)]
pub struct WorkflowDiff {
    pub name_changed: Option<(String, String)>,
    pub active_changed: Option<(bool, bool)>,
    pub nodes_added: Vec<Node>,
    pub nodes_removed: Vec<Node>,
    pub nodes_modified: Vec<NodeDiff>,
    pub connections_added: Vec<Connection>,
    pub connections_removed: Vec<Connection>,
}

#[derive(Debug)]
pub struct NodeDiff {
    pub node_id: String,
    pub node_name: String,
    pub changes: Vec<NodeChange>,
}

#[derive(Debug)]
pub enum NodeChange {
    NameChanged(String, String),
    TypeChanged(String, String),
    PositionChanged((i32, i32), (i32, i32)),
    DisabledChanged(bool, bool),
    ParametersChanged(String), // Unified diff text
}

impl WorkflowDiff {
    /// Compare two workflows
    pub fn compare(old: &TypedWorkflow, new: &TypedWorkflow) -> Self {
        let mut diff = WorkflowDiff::default();

        // Name change
        if old.name != new.name {
            diff.name_changed = Some((old.name.clone(), new.name.clone()));
        }

        // Active change
        if old.active != new.active {
            diff.active_changed = Some((old.active, new.active));
        }

        // Build node maps by ID
        let old_nodes: HashMap<_, _> = old.nodes.iter().map(|n| (n.id.clone(), n)).collect();
        let new_nodes: HashMap<_, _> = new.nodes.iter().map(|n| (n.id.clone(), n)).collect();

        // Find added/removed/modified nodes
        let old_ids: HashSet<_> = old_nodes.keys().collect();
        let new_ids: HashSet<_> = new_nodes.keys().collect();

        for id in new_ids.difference(&old_ids) {
            diff.nodes_added.push(new_nodes[*id].clone());
        }

        for id in old_ids.difference(&new_ids) {
            diff.nodes_removed.push(old_nodes[*id].clone());
        }

        for id in old_ids.intersection(&new_ids) {
            let old_node = old_nodes[*id];
            let new_node = new_nodes[*id];

            if let Some(node_diff) = Self::compare_nodes(old_node, new_node) {
                diff.nodes_modified.push(node_diff);
            }
        }

        // Compare connections
        let old_conns: HashSet<_> = old
            .connections_flat()
            .iter()
            .map(|c| {
                (
                    c.source_node.clone(),
                    c.source_output,
                    c.target_node.clone(),
                    c.target_input,
                )
            })
            .collect();
        let new_conns: HashSet<_> = new
            .connections_flat()
            .iter()
            .map(|c| {
                (
                    c.source_node.clone(),
                    c.source_output,
                    c.target_node.clone(),
                    c.target_input,
                )
            })
            .collect();

        for conn in new.connections_flat() {
            let key = (
                conn.source_node.clone(),
                conn.source_output,
                conn.target_node.clone(),
                conn.target_input,
            );
            if !old_conns.contains(&key) {
                diff.connections_added.push(conn);
            }
        }

        for conn in old.connections_flat() {
            let key = (
                conn.source_node.clone(),
                conn.source_output,
                conn.target_node.clone(),
                conn.target_input,
            );
            if !new_conns.contains(&key) {
                diff.connections_removed.push(conn);
            }
        }

        diff
    }

    fn compare_nodes(old: &Node, new: &Node) -> Option<NodeDiff> {
        let mut changes = Vec::new();

        if old.name != new.name {
            changes.push(NodeChange::NameChanged(old.name.clone(), new.name.clone()));
        }
        if old.node_type != new.node_type {
            changes.push(NodeChange::TypeChanged(
                old.node_type.clone(),
                new.node_type.clone(),
            ));
        }
        if old.position.x != new.position.x || old.position.y != new.position.y {
            changes.push(NodeChange::PositionChanged(
                (old.position.x, old.position.y),
                (new.position.x, new.position.y),
            ));
        }
        if old.disabled != new.disabled {
            changes.push(NodeChange::DisabledChanged(old.disabled, new.disabled));
        }

        // Compare parameters as JSON strings
        let old_params = serde_json::to_string_pretty(&old.parameters).unwrap_or_default();
        let new_params = serde_json::to_string_pretty(&new.parameters).unwrap_or_default();
        if old_params != new_params {
            let text_diff = TextDiff::from_lines(&old_params, &new_params);
            let mut unified = String::new();
            for change in text_diff.iter_all_changes() {
                let sign = match change.tag() {
                    ChangeTag::Delete => "-",
                    ChangeTag::Insert => "+",
                    ChangeTag::Equal => " ",
                };
                unified.push_str(&format!("{}{}", sign, change));
            }
            if !unified.is_empty() {
                changes.push(NodeChange::ParametersChanged(unified));
            }
        }

        if changes.is_empty() {
            None
        } else {
            Some(NodeDiff {
                node_id: old.id.clone(),
                node_name: old.name.clone(),
                changes,
            })
        }
    }

    /// Check if there are any differences
    pub fn is_empty(&self) -> bool {
        self.name_changed.is_none()
            && self.active_changed.is_none()
            && self.nodes_added.is_empty()
            && self.nodes_removed.is_empty()
            && self.nodes_modified.is_empty()
            && self.connections_added.is_empty()
            && self.connections_removed.is_empty()
    }

    /// Print summary to stdout
    pub fn print_summary(&self) {
        if self.is_empty() {
            println!("No differences found.");
            return;
        }

        if let Some((old, new)) = &self.name_changed {
            println!("  Name: \"{}\" -> \"{}\"", old, new);
        }

        if let Some((old, new)) = &self.active_changed {
            println!("  Active: {} -> {}", old, new);
        }

        if !self.nodes_added.is_empty() {
            println!("\n+ Added {} node(s):", self.nodes_added.len());
            for node in &self.nodes_added {
                println!("  + {} ({})", node.name, node.node_type);
            }
        }

        if !self.nodes_removed.is_empty() {
            println!("\n- Removed {} node(s):", self.nodes_removed.len());
            for node in &self.nodes_removed {
                println!("  - {} ({})", node.name, node.node_type);
            }
        }

        if !self.nodes_modified.is_empty() {
            println!("\n~ Modified {} node(s):", self.nodes_modified.len());
            for node_diff in &self.nodes_modified {
                println!("  ~ {}", node_diff.node_name);
                for change in &node_diff.changes {
                    match change {
                        NodeChange::NameChanged(old, new) => {
                            println!("    name: \"{}\" -> \"{}\"", old, new);
                        }
                        NodeChange::TypeChanged(old, new) => {
                            println!("    type: {} -> {}", old, new);
                        }
                        NodeChange::PositionChanged(old, new) => {
                            println!(
                                "    position: ({},{}) -> ({},{})",
                                old.0, old.1, new.0, new.1
                            );
                        }
                        NodeChange::DisabledChanged(old, new) => {
                            println!("    disabled: {} -> {}", old, new);
                        }
                        NodeChange::ParametersChanged(_) => {
                            println!("    parameters: (modified)");
                        }
                    }
                }
            }
        }

        if !self.connections_added.is_empty() {
            println!("\n+ Added {} connection(s):", self.connections_added.len());
            for conn in &self.connections_added {
                println!("  + {} -> {}", conn.source_node, conn.target_node);
            }
        }

        if !self.connections_removed.is_empty() {
            println!(
                "\n- Removed {} connection(s):",
                self.connections_removed.len()
            );
            for conn in &self.connections_removed {
                println!("  - {} -> {}", conn.source_node, conn.target_node);
            }
        }
    }

    /// Print full diff including parameter changes
    pub fn print_full(&self) {
        if self.is_empty() {
            println!("No differences found.");
            return;
        }

        self.print_summary();

        // Print parameter diffs for modified nodes
        for node_diff in &self.nodes_modified {
            for change in &node_diff.changes {
                if let NodeChange::ParametersChanged(diff_text) = change {
                    println!("\n--- {} parameters ---", node_diff.node_name);
                    println!("{}", diff_text);
                }
            }
        }
    }
}
