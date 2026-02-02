use crate::error::{N8nError, Result};
use crate::models::TypedWorkflow;
use std::env;
use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

/// Open workflow in external editor and return the edited version
pub fn edit_workflow(workflow: &TypedWorkflow, editor: Option<&str>) -> Result<TypedWorkflow> {
    // 1. Determine editor
    let editor = editor
        .map(String::from)
        .or_else(|| env::var("EDITOR").ok())
        .or_else(|| env::var("VISUAL").ok())
        .unwrap_or_else(|| "vi".to_string());

    // 2. Create temp file with workflow JSON
    let temp_file = NamedTempFile::with_suffix(".json").map_err(|e| N8nError::FileWrite {
        path: "temp file".to_string(),
        source: e,
    })?;

    let content =
        serde_json::to_string_pretty(workflow).map_err(N8nError::Serialize)?;

    fs::write(temp_file.path(), &content).map_err(|e| N8nError::FileWrite {
        path: temp_file.path().display().to_string(),
        source: e,
    })?;

    // 3. Get file modification time before edit
    let before_modified = fs::metadata(temp_file.path())
        .ok()
        .and_then(|m| m.modified().ok());

    // 4. Open editor
    let status = Command::new(&editor)
        .arg(temp_file.path())
        .status()
        .map_err(|e| N8nError::EditorFailed(format!("Failed to launch editor '{}': {}", editor, e)))?;

    if !status.success() {
        return Err(N8nError::EditorFailed(format!(
            "Editor '{}' exited with status {}",
            editor,
            status.code().unwrap_or(-1)
        )));
    }

    // 5. Check if file was modified
    let after_modified = fs::metadata(temp_file.path())
        .ok()
        .and_then(|m| m.modified().ok());

    if before_modified == after_modified {
        return Err(N8nError::NoChanges);
    }

    // 6. Read modified content
    let modified_content = fs::read_to_string(temp_file.path()).map_err(|e| N8nError::FileRead {
        path: temp_file.path().display().to_string(),
        source: e,
    })?;

    // 7. Parse and return
    let edited: TypedWorkflow =
        serde_json::from_str(&modified_content).map_err(N8nError::InvalidInput)?;

    Ok(edited)
}
