use clap::Parser;
use n8n_cli::{
    cli::{
        Cli, Commands, ConnectionsAction, CredentialsAction, ExecutionsAction, HealthAction,
        NodesAction, TagsAction, WorkflowsAction,
    },
    client::{
        endpoints::{
            credentials::CredentialListParams, executions::ExecutionListParams,
            workflows::WorkflowListParams,
        },
        N8nClient,
    },
    config::{load_config, validate_config, CliOverrides},
    diff::WorkflowDiff,
    editor::edit_workflow,
    error::{N8nError, Result},
    models::{Connection, CredentialCreate, Node, Position, TypedWorkflow, WorkflowDefinition},
    output::{print_output, print_single},
    validation::validate_workflow,
};
use serde_json::Value;
use std::io::{self, Read};
use std::path::Path;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        std::process::exit(e.exit_code());
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration with layering
    let overrides = CliOverrides {
        profile: cli.profile.clone(),
        url: cli.url.clone(),
        api_key: cli.api_key.clone(),
        output: cli.output,
        verbose: cli.verbose,
        quiet: cli.quiet,
    };
    let config = load_config(overrides)?;

    // Handle config command separately (doesn't need API key)
    if let Commands::Config = cli.command {
        return handle_config(&config);
    }

    // Handle validate --file separately (doesn't need API key)
    if let Commands::Workflows(ref cmd) = cli.command {
        if let WorkflowsAction::Validate {
            id: None,
            file: Some(ref path),
            warnings,
        } = cmd.action
        {
            return handle_validate_file(path, warnings);
        }
    }

    // Validate configuration for API commands
    validate_config(&config)?;

    // Create API client
    let client = N8nClient::new(&config)?;

    // Dispatch to appropriate handler
    match cli.command {
        Commands::Workflows(cmd) => handle_workflows(&client, cmd.action, &config).await,
        Commands::Executions(cmd) => handle_executions(&client, cmd.action, &config).await,
        Commands::Credentials(cmd) => handle_credentials(&client, cmd.action, &config).await,
        Commands::Tags(cmd) => handle_tags(&client, cmd.action, &config).await,
        Commands::Health(cmd) => handle_health(&client, cmd.action, &config).await,
        Commands::Config => unreachable!(), // Handled above
    }
}

fn handle_config(config: &n8n_cli::Config) -> Result<()> {
    println!("Current configuration:");
    println!("  Base URL: {}", config.base_url);
    println!(
        "  API Key:  {}",
        if config.api_key.is_empty() {
            "(not set)"
        } else {
            "(set)"
        }
    );
    println!("  Output:   {:?}", config.output_format);
    println!("  Timeout:  {}s", config.timeout_secs);
    Ok(())
}

fn handle_validate_file(path: &Path, warnings: bool) -> Result<()> {
    let content = read_file_or_stdin(path)?;
    let workflow: TypedWorkflow =
        serde_json::from_str(&content).map_err(N8nError::InvalidInput)?;

    let result = validate_workflow(&workflow);

    if result.is_valid() {
        println!("Workflow is valid.");
        if warnings && !result.warnings().is_empty() {
            println!("\nWarnings:");
            println!("{}", result.format(true));
        }
        Ok(())
    } else {
        Err(N8nError::ValidationFailed(result.format(warnings)))
    }
}

async fn handle_workflows(
    client: &N8nClient,
    action: WorkflowsAction,
    config: &n8n_cli::Config,
) -> Result<()> {
    match action {
        WorkflowsAction::List {
            active,
            tags,
            name,
            limit,
            cursor,
            all,
        } => {
            let params = WorkflowListParams {
                limit: Some(limit),
                cursor,
                active,
                tags: tags.map(|t| t.join(",")),
                name,
            };

            if all {
                let workflows = client.list_all_workflows(params).await?;
                print_output(&workflows, config.output_format)?;
            } else {
                let response = client.list_workflows(&params).await?;
                print_output(&response.data, config.output_format)?;
                if let Some(next) = response.next_cursor {
                    if !config.quiet {
                        eprintln!("\nMore results available. Use --cursor {} to continue", next);
                    }
                }
            }
        }

        WorkflowsAction::Get { id } => {
            let workflow = client.get_workflow(&id).await?;
            print_single(&workflow, config.output_format)?;
        }

        WorkflowsAction::Create { file, activate } => {
            let content = read_file_or_stdin(&file)?;
            let workflow: WorkflowDefinition =
                serde_json::from_str(&content).map_err(N8nError::InvalidInput)?;

            let created = client.create_workflow(&workflow).await?;
            if !config.quiet {
                eprintln!("Created workflow: {} ({})", created.name, created.id);
            }

            if activate {
                let activated = client.activate_workflow(&created.id).await?;
                if !config.quiet {
                    eprintln!("Activated workflow: {}", activated.id);
                }
                print_single(&activated, config.output_format)?;
            } else {
                print_single(&created, config.output_format)?;
            }
        }

        WorkflowsAction::Update { id, file } => {
            let content = read_file_or_stdin(&file)?;
            let workflow: WorkflowDefinition =
                serde_json::from_str(&content).map_err(N8nError::InvalidInput)?;

            let updated = client.update_workflow(&id, &workflow).await?;
            if !config.quiet {
                eprintln!("Updated workflow: {} ({})", updated.name, updated.id);
            }
            print_single(&updated, config.output_format)?;
        }

        WorkflowsAction::Delete { id, force } => {
            if !force {
                eprint!("Delete workflow {}? [y/N] ", id);
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .map_err(N8nError::StdinRead)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    return Err(N8nError::Cancelled);
                }
            }

            client.delete_workflow(&id).await?;
            if !config.quiet {
                eprintln!("Deleted workflow: {}", id);
            }
        }

        WorkflowsAction::Activate { id } => {
            let workflow = client.activate_workflow(&id).await?;
            if !config.quiet {
                eprintln!("Activated workflow: {} ({})", workflow.name, workflow.id);
            }
            print_single(&workflow, config.output_format)?;
        }

        WorkflowsAction::Deactivate { id } => {
            let workflow = client.deactivate_workflow(&id).await?;
            if !config.quiet {
                eprintln!("Deactivated workflow: {} ({})", workflow.name, workflow.id);
            }
            print_single(&workflow, config.output_format)?;
        }

        WorkflowsAction::Nodes(cmd) => {
            handle_nodes(client, cmd.action, config).await?;
        }

        WorkflowsAction::Connections(cmd) => {
            handle_connections(client, cmd.action, config).await?;
        }

        WorkflowsAction::Edit {
            id,
            editor,
            no_validate,
        } => {
            // Fetch workflow
            let detail = client.get_workflow(&id).await?;
            let workflow = TypedWorkflow::from_detail(detail)?;

            // Open in editor
            let edited = edit_workflow(&workflow, editor.as_deref())?;

            // Validate if requested
            if !no_validate {
                let result = validate_workflow(&edited);
                if !result.is_valid() {
                    return Err(N8nError::ValidationFailed(result.format(true)));
                }
            }

            // Update
            let definition = edited.to_definition();
            let updated = client.update_workflow(&id, &definition).await?;

            if !config.quiet {
                eprintln!("Updated workflow: {} ({})", updated.name, updated.id);
            }
            print_single(&updated, config.output_format)?;
        }

        WorkflowsAction::Diff { id, with, file, full } => {
            // Get first workflow
            let detail1 = client.get_workflow(&id).await?;
            let workflow1 = TypedWorkflow::from_detail(detail1)?;

            // Get second workflow (from id or file)
            let workflow2 = if let Some(with_id) = with {
                let detail2 = client.get_workflow(&with_id).await?;
                TypedWorkflow::from_detail(detail2)?
            } else if let Some(path) = file {
                let content = read_file_or_stdin(&path)?;
                serde_json::from_str(&content).map_err(N8nError::InvalidInput)?
            } else {
                return Err(N8nError::Config(
                    "Either --with or --file must be specified".to_string(),
                ));
            };

            let diff = WorkflowDiff::compare(&workflow1, &workflow2);

            if full {
                diff.print_full();
            } else {
                diff.print_summary();
            }
        }

        WorkflowsAction::Export { id, file, pretty } => {
            let detail = client.get_workflow(&id).await?;
            let workflow = TypedWorkflow::from_detail(detail)?;

            let content = if pretty {
                serde_json::to_string_pretty(&workflow).map_err(N8nError::Serialize)?
            } else {
                serde_json::to_string(&workflow).map_err(N8nError::Serialize)?
            };

            if let Some(path) = file {
                std::fs::write(&path, &content).map_err(|e| N8nError::FileWrite {
                    path: path.display().to_string(),
                    source: e,
                })?;
                if !config.quiet {
                    eprintln!("Exported workflow to: {}", path.display());
                }
            } else {
                println!("{}", content);
            }
        }

        WorkflowsAction::Clone { id, name, activate } => {
            let detail = client.get_workflow(&id).await?;
            let mut workflow = TypedWorkflow::from_detail(detail)?;

            // Set new name and remove ID
            workflow.name = name.clone();
            workflow.id = None;
            workflow.active = false;

            let definition = workflow.to_definition();
            let created = client.create_workflow(&definition).await?;

            if !config.quiet {
                eprintln!("Cloned workflow: {} ({})", created.name, created.id);
            }

            if activate {
                let activated = client.activate_workflow(&created.id).await?;
                if !config.quiet {
                    eprintln!("Activated workflow: {}", activated.id);
                }
                print_single(&activated, config.output_format)?;
            } else {
                print_single(&created, config.output_format)?;
            }
        }

        WorkflowsAction::Validate { id, file, warnings } => {
            // file-only case is handled at the top of run()
            if let Some(workflow_id) = id {
                let detail = client.get_workflow(&workflow_id).await?;
                let workflow = TypedWorkflow::from_detail(detail)?;

                let result = validate_workflow(&workflow);

                if result.is_valid() {
                    println!("Workflow '{}' is valid.", workflow.name);
                    if warnings && !result.warnings().is_empty() {
                        println!("\nWarnings:");
                        println!("{}", result.format(true));
                    }
                } else {
                    return Err(N8nError::ValidationFailed(result.format(warnings)));
                }
            } else if file.is_none() {
                return Err(N8nError::Config(
                    "Either workflow ID or --file must be specified".to_string(),
                ));
            }
        }

        WorkflowsAction::Run {
            id,
            data,
            data_file,
            method,
            no_wait,
        } => {
            // Get workflow to find webhook path
            let detail = client.get_workflow(&id).await?;
            let workflow = TypedWorkflow::from_detail(detail)?;

            // Find webhook node
            let webhook_node = workflow.nodes.iter().find(|n| {
                n.node_type.contains("webhook") || n.node_type.contains("formTrigger")
            });

            let webhook_path = match webhook_node {
                Some(node) => {
                    // Extract path from parameters
                    node.parameters
                        .get("path")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| id.clone())
                }
                None => {
                    return Err(N8nError::Config(
                        "Workflow has no webhook trigger. Only webhook workflows can be run via CLI.\n\
                         Hint: Add a Webhook or Form Trigger node, or run manually in n8n UI."
                            .to_string(),
                    ));
                }
            };

            // Build webhook URL
            let webhook_url = format!(
                "{}/webhook{}{}",
                config.base_url.trim_end_matches('/'),
                if workflow.active { "" } else { "-test" },
                if webhook_path.starts_with('/') {
                    webhook_path.clone()
                } else {
                    format!("/{}", webhook_path)
                }
            );

            // Parse input data
            let body: Option<Value> = if let Some(json_data) = data {
                Some(serde_json::from_str(&json_data).map_err(N8nError::InvalidInput)?)
            } else if let Some(path) = data_file {
                let content = read_file_or_stdin(&path)?;
                Some(serde_json::from_str(&content).map_err(N8nError::InvalidInput)?)
            } else {
                None
            };

            if !config.quiet {
                eprintln!(
                    "Triggering workflow '{}' via {}...",
                    workflow.name,
                    if workflow.active { "webhook" } else { "test webhook" }
                );
                eprintln!("URL: {}", webhook_url);
            }

            // Make webhook request
            let http_client = reqwest::Client::new();
            let mut request = match method.to_uppercase().as_str() {
                "GET" => http_client.get(&webhook_url),
                "POST" => http_client.post(&webhook_url),
                "PUT" => http_client.put(&webhook_url),
                "PATCH" => http_client.patch(&webhook_url),
                "DELETE" => http_client.delete(&webhook_url),
                _ => http_client.post(&webhook_url),
            };

            if let Some(body_data) = body {
                request = request.json(&body_data);
            }

            let response = request.send().await.map_err(N8nError::Request)?;
            let status = response.status();

            if status.is_success() {
                if !config.quiet {
                    eprintln!("Workflow triggered successfully ({})", status);
                }

                if !no_wait {
                    let response_body: Value = response.json().await.unwrap_or(Value::Null);
                    print_single(&response_body, config.output_format)?;
                }
            } else {
                let error_text = response.text().await.unwrap_or_default();
                return Err(N8nError::Config(format!(
                    "Webhook request failed ({}): {}",
                    status, error_text
                )));
            }
        }
    }

    Ok(())
}

async fn handle_nodes(
    client: &N8nClient,
    action: NodesAction,
    config: &n8n_cli::Config,
) -> Result<()> {
    match action {
        NodesAction::List { workflow_id } => {
            let detail = client.get_workflow(&workflow_id).await?;
            let workflow = TypedWorkflow::from_detail(detail)?;
            print_output(&workflow.nodes, config.output_format)?;
        }

        NodesAction::Get {
            workflow_id,
            node_id,
        } => {
            let detail = client.get_workflow(&workflow_id).await?;
            let workflow = TypedWorkflow::from_detail(detail)?;

            let node = workflow
                .find_node(&node_id)
                .ok_or_else(|| N8nError::NodeNotFound(node_id.clone()))?;

            print_single(node, config.output_format)?;
        }

        NodesAction::Add {
            workflow_id,
            r#type,
            name,
            position,
            config: node_config,
            config_file,
            disabled,
        } => {
            let detail = client.get_workflow(&workflow_id).await?;
            let mut workflow = TypedWorkflow::from_detail(detail)?;

            // Parse config
            let parameters: Value = if let Some(cfg) = node_config {
                serde_json::from_str(&cfg).map_err(N8nError::InvalidInput)?
            } else if let Some(path) = config_file {
                let content = read_file_or_stdin(&path)?;
                serde_json::from_str(&content).map_err(N8nError::InvalidInput)?
            } else {
                Value::Object(serde_json::Map::new())
            };

            // Create node
            let mut node = Node::new(Node::generate_id(), name.clone(), r#type);
            node.parameters = parameters;
            node.disabled = disabled;

            if let Some((x, y)) = position {
                node.position = Position::new(x, y);
            } else {
                node.position = workflow.auto_position();
            }

            workflow.add_node(node.clone());

            // Update workflow
            let definition = workflow.to_definition();
            client.update_workflow(&workflow_id, &definition).await?;

            if !config.quiet {
                eprintln!("Added node '{}' ({}) to workflow", node.name, node.id);
            }
            print_single(&node, config.output_format)?;
        }

        NodesAction::Remove {
            workflow_id,
            node_id,
            force,
        } => {
            let detail = client.get_workflow(&workflow_id).await?;
            let mut workflow = TypedWorkflow::from_detail(detail)?;

            // Check node exists
            let node_name = workflow
                .find_node(&node_id)
                .map(|n| n.name.clone())
                .ok_or_else(|| N8nError::NodeNotFound(node_id.clone()))?;

            if !force {
                eprint!("Remove node '{}'? [y/N] ", node_name);
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .map_err(N8nError::StdinRead)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    return Err(N8nError::Cancelled);
                }
            }

            workflow.remove_node(&node_id);

            // Update workflow
            let definition = workflow.to_definition();
            client.update_workflow(&workflow_id, &definition).await?;

            if !config.quiet {
                eprintln!("Removed node '{}' from workflow", node_name);
            }
        }

        NodesAction::Update {
            workflow_id,
            node_id,
            name,
            position,
            config: node_config,
            replace,
            disabled,
        } => {
            let detail = client.get_workflow(&workflow_id).await?;
            let mut workflow = TypedWorkflow::from_detail(detail)?;

            let node = workflow
                .find_node_mut(&node_id)
                .ok_or_else(|| N8nError::NodeNotFound(node_id.clone()))?;

            let old_name = node.name.clone();

            if let Some(ref new_name) = name {
                node.name = new_name.clone();
            }

            if let Some((x, y)) = position {
                node.position = Position::new(x, y);
            }

            if let Some(cfg) = node_config {
                let new_params: Value = serde_json::from_str(&cfg).map_err(N8nError::InvalidInput)?;
                if replace {
                    node.parameters = new_params;
                } else {
                    // Merge parameters
                    if let (Value::Object(existing), Value::Object(new)) =
                        (&mut node.parameters, new_params)
                    {
                        for (k, v) in new {
                            existing.insert(k, v);
                        }
                    }
                }
            }

            if let Some(dis) = disabled {
                node.disabled = dis;
            }

            let node_name = node.name.clone();

            // Update connection references if node was renamed
            if let Some(ref new_name) = name {
                if old_name != *new_name {
                    workflow.rename_node_in_connections(&old_name, new_name);
                }
            }

            // Update workflow
            let definition = workflow.to_definition();
            client.update_workflow(&workflow_id, &definition).await?;

            if !config.quiet {
                eprintln!("Updated node '{}'", node_name);
            }
        }

        NodesAction::Move {
            workflow_id,
            node_id,
            position,
        } => {
            let detail = client.get_workflow(&workflow_id).await?;
            let mut workflow = TypedWorkflow::from_detail(detail)?;

            let node = workflow
                .find_node_mut(&node_id)
                .ok_or_else(|| N8nError::NodeNotFound(node_id.clone()))?;

            node.position = Position::new(position.0, position.1);
            let node_name = node.name.clone();

            // Update workflow
            let definition = workflow.to_definition();
            client.update_workflow(&workflow_id, &definition).await?;

            if !config.quiet {
                eprintln!(
                    "Moved node '{}' to ({},{})",
                    node_name, position.0, position.1
                );
            }
        }
    }

    Ok(())
}

async fn handle_connections(
    client: &N8nClient,
    action: ConnectionsAction,
    config: &n8n_cli::Config,
) -> Result<()> {
    match action {
        ConnectionsAction::List {
            workflow_id,
            from,
            to,
        } => {
            let detail = client.get_workflow(&workflow_id).await?;
            let workflow = TypedWorkflow::from_detail(detail)?;

            let mut connections = workflow.connections_flat();

            // Filter if requested
            if let Some(from_node) = from {
                let from_name = workflow
                    .get_node_name(&from_node)
                    .unwrap_or_else(|| from_node.clone());
                connections.retain(|c| c.source_node == from_name);
            }
            if let Some(to_node) = to {
                let to_name = workflow
                    .get_node_name(&to_node)
                    .unwrap_or_else(|| to_node.clone());
                connections.retain(|c| c.target_node == to_name);
            }

            print_output(&connections, config.output_format)?;
        }

        ConnectionsAction::Add {
            workflow_id,
            from,
            to,
            output_index,
            input_index,
            r#type,
        } => {
            let detail = client.get_workflow(&workflow_id).await?;
            let mut workflow = TypedWorkflow::from_detail(detail)?;

            // Resolve node names (n8n uses names in connections)
            let from_name = workflow
                .get_node_name(&from)
                .ok_or_else(|| N8nError::NodeNotFound(from.clone()))?;
            let to_name = workflow
                .get_node_name(&to)
                .ok_or_else(|| N8nError::NodeNotFound(to.clone()))?;

            let conn = Connection::new_full(
                from_name.clone(),
                output_index,
                r#type.clone(),
                to_name.clone(),
                input_index,
                r#type,
            );

            workflow.add_connection(conn);

            // Update workflow
            let definition = workflow.to_definition();
            client.update_workflow(&workflow_id, &definition).await?;

            if !config.quiet {
                eprintln!("Added connection: {} -> {}", from_name, to_name);
            }
        }

        ConnectionsAction::Remove {
            workflow_id,
            from,
            to,
            force,
        } => {
            let detail = client.get_workflow(&workflow_id).await?;
            let mut workflow = TypedWorkflow::from_detail(detail)?;

            // Resolve node names
            let from_name = workflow.get_node_name(&from).unwrap_or_else(|| from.clone());
            let to_name = workflow.get_node_name(&to).unwrap_or_else(|| to.clone());

            if !force {
                eprint!("Remove connection {} -> {}? [y/N] ", from_name, to_name);
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .map_err(N8nError::StdinRead)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    return Err(N8nError::Cancelled);
                }
            }

            let removed = workflow.remove_connection(&from_name, &to_name);
            if !removed {
                return Err(N8nError::ConnectionNotFound {
                    from: from_name,
                    to: to_name,
                });
            }

            // Update workflow
            let definition = workflow.to_definition();
            client.update_workflow(&workflow_id, &definition).await?;

            if !config.quiet {
                eprintln!("Removed connection: {} -> {}", from_name, to_name);
            }
        }
    }

    Ok(())
}

async fn handle_executions(
    client: &N8nClient,
    action: ExecutionsAction,
    config: &n8n_cli::Config,
) -> Result<()> {
    match action {
        ExecutionsAction::List {
            workflow_id,
            status,
            include_data,
            limit,
            cursor,
        } => {
            let params = ExecutionListParams {
                limit: Some(limit),
                cursor,
                workflow_id,
                status,
                include_data: if include_data { Some(true) } else { None },
            };

            let response = client.list_executions(&params).await?;
            print_output(&response.data, config.output_format)?;
            if let Some(next) = response.next_cursor {
                if !config.quiet {
                    eprintln!("\nMore results available. Use --cursor {} to continue", next);
                }
            }
        }

        ExecutionsAction::Get { id, include_data } => {
            let execution = client.get_execution(&id, include_data).await?;
            print_single(&execution, config.output_format)?;
        }

        ExecutionsAction::Delete { id } => {
            client.delete_execution(&id).await?;
            if !config.quiet {
                eprintln!("Deleted execution: {}", id);
            }
        }

        ExecutionsAction::Retry { id } => {
            let execution = client.retry_execution(&id).await?;
            if !config.quiet {
                eprintln!("Retried execution: {}", execution.id);
            }
            print_single(&execution, config.output_format)?;
        }

        ExecutionsAction::Run {
            workflow_id,
            wait: _,
            data,
        } => {
            let input_data = match data {
                Some(json_str) => {
                    Some(serde_json::from_str(&json_str).map_err(N8nError::InvalidInput)?)
                }
                None => None,
            };

            let execution = client.execute_workflow(&workflow_id, input_data).await?;
            if !config.quiet {
                eprintln!("Started execution: {}", execution.id);
            }
            print_single(&execution, config.output_format)?;
        }
    }

    Ok(())
}

async fn handle_credentials(
    client: &N8nClient,
    action: CredentialsAction,
    config: &n8n_cli::Config,
) -> Result<()> {
    match action {
        CredentialsAction::List { r#type } => {
            let params = CredentialListParams {
                limit: Some(100),
                cursor: None,
                credential_type: r#type,
            };

            let response = client.list_credentials(&params).await?;
            print_output(&response.data, config.output_format)?;
        }

        CredentialsAction::Schema { type_name } => {
            let schema = client.get_credential_schema(&type_name).await?;
            print_single(&schema, config.output_format)?;
        }

        CredentialsAction::Create { file } => {
            let content = read_file_or_stdin(&file)?;
            let credential: CredentialCreate =
                serde_json::from_str(&content).map_err(N8nError::InvalidInput)?;

            let created = client.create_credential(&credential).await?;
            if !config.quiet {
                eprintln!("Created credential: {} ({})", created.name, created.id);
            }
            print_single(&created, config.output_format)?;
        }

        CredentialsAction::Update { id, file } => {
            let content = read_file_or_stdin(&file)?;
            let credential: CredentialCreate =
                serde_json::from_str(&content).map_err(N8nError::InvalidInput)?;

            let updated = client.update_credential(&id, &credential).await?;
            if !config.quiet {
                eprintln!("Updated credential: {} ({})", updated.name, updated.id);
            }
            print_single(&updated, config.output_format)?;
        }

        CredentialsAction::Delete { id, force } => {
            if !force {
                eprint!("Delete credential {}? [y/N] ", id);
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .map_err(N8nError::StdinRead)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    return Err(N8nError::Cancelled);
                }
            }

            client.delete_credential(&id).await?;
            if !config.quiet {
                eprintln!("Deleted credential: {}", id);
            }
        }
    }

    Ok(())
}

async fn handle_tags(
    client: &N8nClient,
    action: TagsAction,
    config: &n8n_cli::Config,
) -> Result<()> {
    match action {
        TagsAction::List => {
            let tags = client.list_tags().await?;
            print_output(&tags, config.output_format)?;
        }

        TagsAction::Create { name } => {
            let tag = client.create_tag(&name).await?;
            if !config.quiet {
                eprintln!("Created tag: {} ({})", tag.name, tag.id);
            }
            print_single(&tag, config.output_format)?;
        }

        TagsAction::Update { id, name } => {
            let tag = client.update_tag(&id, &name).await?;
            if !config.quiet {
                eprintln!("Updated tag: {} ({})", tag.name, tag.id);
            }
            print_single(&tag, config.output_format)?;
        }

        TagsAction::Delete { id } => {
            client.delete_tag(&id).await?;
            if !config.quiet {
                eprintln!("Deleted tag: {}", id);
            }
        }

        TagsAction::Assign { workflow_id, tags } => {
            // First, get existing tags to resolve names to IDs
            let all_tags = client.list_tags().await?;
            let mut tag_ids = Vec::new();

            for tag_name in &tags {
                if let Some(tag) =
                    all_tags.iter().find(|t| &t.name == tag_name || &t.id == tag_name)
                {
                    tag_ids.push(tag.id.clone());
                } else {
                    return Err(N8nError::Config(format!("Tag not found: {}", tag_name)));
                }
            }

            client.assign_tags(&workflow_id, tag_ids).await?;
            if !config.quiet {
                eprintln!(
                    "Assigned tags [{}] to workflow {}",
                    tags.join(", "),
                    workflow_id
                );
            }
        }
    }

    Ok(())
}

async fn handle_health(
    client: &N8nClient,
    action: HealthAction,
    config: &n8n_cli::Config,
) -> Result<()> {
    match action {
        HealthAction::Check => {
            let health = client.health_check().await?;
            if config.quiet {
                println!("{}", health.status);
            } else {
                println!("Health: {}", health.status);
            }
        }

        HealthAction::Ready => {
            let ready = client.readiness_check().await?;
            if config.quiet {
                println!("{}", ready.status);
            } else {
                println!("Readiness: {}", ready.status);
            }
        }
    }

    Ok(())
}

/// Read content from a file or stdin if path is "-"
fn read_file_or_stdin(path: &Path) -> Result<String> {
    if path.as_os_str() == "-" {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(N8nError::StdinRead)?;
        Ok(buffer)
    } else {
        std::fs::read_to_string(path).map_err(|e| N8nError::FileRead {
            path: path.display().to_string(),
            source: e,
        })
    }
}
