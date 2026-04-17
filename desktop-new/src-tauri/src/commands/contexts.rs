use crate::daemon::cli::CliRunner;
use crate::daemon::state::DaemonState;
use crate::daemon::types::Context;
use crate::persistence::audit::AuditLog;
use log::error;
use serde::Serialize;
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

type SharedState = Arc<RwLock<DaemonState>>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextListResult {
    pub contexts: Vec<Context>,
    pub active_context: String,
}

#[tauri::command]
pub async fn context_list(state: State<'_, SharedState>) -> Result<ContextListResult, String> {
    let state = state.read().await;
    Ok(ContextListResult {
        contexts: state.contexts.clone(),
        active_context: state.active_context.clone(),
    })
}

#[tauri::command]
pub async fn context_use(
    cli: State<'_, Arc<CliRunner>>,
    audit: State<'_, Arc<AuditLog>>,
    name: String,
) -> Result<(), String> {
    let result = cli.run_raw(&["context", "use", &name]).await;
    let success = result.is_ok();
    if let Err(e) = audit.record("use", "context", &name, "", success) {
        error!("Failed to record audit entry: {}", e);
    }
    result.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn context_options(
    cli: State<'_, Arc<CliRunner>>,
    context: Option<String>,
) -> Result<serde_json::Value, String> {
    let mut args = vec!["context", "options"];
    let ctx;
    if let Some(ref name) = context {
        ctx = name.clone();
        args.push("--name");
        args.push(&ctx);
    }
    cli.run::<serde_json::Value>(&args)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn context_set_options(
    cli: State<'_, Arc<CliRunner>>,
    options: Vec<String>,
    context: Option<String>,
) -> Result<(), String> {
    let mut args: Vec<&str> = vec!["context", "set-options"];
    let ctx;
    if let Some(ref name) = context {
        ctx = name.clone();
        args.push("--name");
        args.push(&ctx);
    }
    let option_refs: Vec<&str> = options.iter().map(|s| s.as_str()).collect();
    for opt in &option_refs {
        args.push("-o");
        args.push(opt);
    }
    cli.run_raw(&args).await.map_err(|e| e.to_string())?;
    Ok(())
}
