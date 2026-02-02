# Claude Skill for n8n Workflow Development

The n8n CLI includes a Claude Code skill that helps you develop, manage, and debug n8n workflows using natural language.

## Installation

### Quick Install

From the n8n-cli repository:

```bash
./install-claude-skill
```

Or if you've added n8n-cli to your PATH:

```bash
install-claude-skill
```

### Manual Install

Copy the skill to your personal Claude skills directory:

```bash
cp -r .claude/skills/n8n ~/.claude/skills/
```

### Verify Installation

```bash
ls ~/.claude/skills/n8n/
# Should show: SKILL.md, examples/
```

## Usage

### Slash Command

Invoke the skill directly with `/n8n`:

```
/n8n create a workflow that sends Slack notifications when a webhook is triggered

/n8n validate my-workflow.json

/n8n add an HTTP Request node to workflow wf_abc123

/n8n debug why workflow wf_xyz789 is failing
```

### Natural Language

Just ask Claude about n8n workflows and it will automatically use the skill:

```
"Help me create an n8n workflow that syncs data from an API to PostgreSQL every hour"

"How do I add error handling to my workflow?"

"What's wrong with this workflow JSON?"

"List all my active workflows"
```

## Capabilities

The skill helps Claude understand and assist with:

### Workflow Creation
- Design workflow structure from requirements
- Generate valid workflow JSON with proper UUIDs
- Choose appropriate node types
- Configure node parameters
- Set up connections between nodes

### Workflow Management
- List, get, create, update, delete workflows
- Activate/deactivate workflows
- Clone workflows
- Export/import workflows
- Tag workflows for organization

### Node Operations
- Add nodes with proper configuration
- Remove nodes (and their connections)
- Update node parameters
- Move/reposition nodes
- List nodes in a workflow

### Connection Management
- Wire nodes together
- Handle branching (IF nodes with multiple outputs)
- Remove connections
- Validate connection integrity

### Execution & Testing
- Run workflows with test data
- Monitor execution status
- Debug failed executions
- Retry failed executions
- View execution logs

### Validation & Debugging
- Validate workflow structure
- Check for common issues
- Compare workflow versions
- Identify orphan nodes
- Detect invalid connections

## Example Interactions

### Create a Workflow

**You:** Create a workflow that receives webhooks and stores data in PostgreSQL

**Claude:** I'll create a webhook-to-database workflow for you.

```bash
# First, let me create the workflow JSON
```

*Creates workflow.json with Webhook trigger, data transformation, and PostgreSQL insert nodes*

```bash
n8n wf create webhook-to-db.json --activate
```

### Debug a Failure

**You:** Why is workflow wf_abc123 failing?

**Claude:** Let me check the recent executions.

```bash
n8n exec list -w wf_abc123 --status error --limit 5
n8n exec get <execution_id> --include-data
```

*Analyzes the error and provides solution*

### Add Error Handling

**You:** Add error handling to my API workflow

**Claude:** I'll add a try-catch pattern with error notifications.

```bash
n8n wf nodes add wf_abc123 -t "n8n-nodes-base.if" -n "Check Error" --position "600,300"
n8n wf nodes add wf_abc123 -t "n8n-nodes-base.code" -n "Handle Error" --position "800,400"
n8n wf connections add wf_abc123 --from "API Call" --to "Check Error"
# ... additional configuration
```

## Skill Files

```
~/.claude/skills/n8n/
├── SKILL.md                      # Main skill instructions
└── examples/
    ├── webhook-to-slack.json     # Webhook notification example
    ├── data-sync-schedule.json   # Scheduled sync example
    └── error-handling.json       # Error handling pattern
```

## Customization

You can extend the skill by editing `~/.claude/skills/n8n/SKILL.md`:

- Add your organization's workflow patterns
- Include company-specific node configurations
- Add custom validation rules
- Include internal API documentation

## Uninstall

Remove the skill:

```bash
rm -rf ~/.claude/skills/n8n
```

## Troubleshooting

### Skill Not Found

If `/n8n` doesn't work:

1. Check the skill is installed: `ls ~/.claude/skills/n8n/SKILL.md`
2. Restart Claude Code
3. Verify SKILL.md has valid YAML frontmatter

### Commands Not Executing

Ensure the n8n CLI is installed and configured:

```bash
n8n --help
n8n health check
```

### Permission Issues

The skill needs access to Bash to run n8n commands. If commands are blocked, check your Claude Code permissions.

## See Also

- [n8n CLI Documentation](../README.md)
- [Workflow Commands](../commands/workflows.md)
- [Running Workflows](./running-workflows.md)
- [Scripting Guide](./scripting.md)
