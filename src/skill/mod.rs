use crate::error::{N8nError, Result};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

// Embed skill files at compile time
const SKILL_MD: &str = include_str!("../../.claude/skills/n8n/SKILL.md");
const EXAMPLE_WEBHOOK_SLACK: &str =
    include_str!("../../.claude/skills/n8n/examples/webhook-to-slack.json");
const EXAMPLE_DATA_SYNC: &str =
    include_str!("../../.claude/skills/n8n/examples/data-sync-schedule.json");
const EXAMPLE_ERROR_HANDLING: &str =
    include_str!("../../.claude/skills/n8n/examples/error-handling.json");

/// Get the Claude skills directory path
fn get_skills_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| N8nError::Config("Could not find home directory".to_string()))?;
    Ok(home.join(".claude").join("skills"))
}

/// Install the n8n Claude skill
pub fn install_claude_skill(force: bool, quiet: bool) -> Result<()> {
    let skills_dir = get_skills_dir()?;
    let skill_dir = skills_dir.join("n8n");
    let examples_dir = skill_dir.join("examples");

    // Check if skill already exists
    if skill_dir.exists() && !force {
        eprint!("Skill 'n8n' already exists at {}. Overwrite? [y/N] ", skill_dir.display());
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(N8nError::StdinRead)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            if !quiet {
                eprintln!("Installation cancelled.");
            }
            return Ok(());
        }

        // Remove existing
        fs::remove_dir_all(&skill_dir).map_err(|e| N8nError::FileWrite {
            path: skill_dir.display().to_string(),
            source: e,
        })?;
    }

    // Create directories
    fs::create_dir_all(&examples_dir).map_err(|e| N8nError::FileWrite {
        path: examples_dir.display().to_string(),
        source: e,
    })?;

    // Write skill files
    let files = [
        (skill_dir.join("SKILL.md"), SKILL_MD),
        (examples_dir.join("webhook-to-slack.json"), EXAMPLE_WEBHOOK_SLACK),
        (examples_dir.join("data-sync-schedule.json"), EXAMPLE_DATA_SYNC),
        (examples_dir.join("error-handling.json"), EXAMPLE_ERROR_HANDLING),
    ];

    for (path, content) in &files {
        fs::write(path, content).map_err(|e| N8nError::FileWrite {
            path: path.display().to_string(),
            source: e,
        })?;
    }

    if !quiet {
        eprintln!();
        eprintln!("✓ Claude skill 'n8n' installed successfully!");
        eprintln!();
        eprintln!("Location: {}", skill_dir.display());
        eprintln!();
        eprintln!("Usage in Claude Code:");
        eprintln!("  /n8n create my-workflow");
        eprintln!("  /n8n validate workflow.json");
        eprintln!("  /n8n debug <workflow-id>");
        eprintln!();
        eprintln!("Or ask Claude naturally about n8n workflows.");
        eprintln!();
    }

    Ok(())
}
