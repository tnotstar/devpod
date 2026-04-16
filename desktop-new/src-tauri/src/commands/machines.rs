use crate::daemon::cli::CliRunner;
use crate::daemon::state::DaemonState;
use crate::daemon::types::Machine;
use crate::persistence::audit::AuditLog;
use log::error;
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

type SharedState = Arc<RwLock<DaemonState>>;

#[tauri::command]
pub async fn machine_list(state: State<'_, SharedState>) -> Result<Vec<Machine>, String> {
    let state = state.read().await;
    Ok(state.machine_list().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn machine_create(
    cli: State<'_, Arc<CliRunner>>,
    audit: State<'_, Arc<AuditLog>>,
    name: String,
    provider: String,
    options: Option<std::collections::HashMap<String, String>>,
) -> Result<(), String> {
    let mut args = vec!["machine", "create", &name, "--provider", &provider];
    let option_args: Vec<String> = options
        .unwrap_or_default()
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    for opt in &option_args {
        args.push("--option");
        args.push(opt);
    }
    let result = cli.run_raw(&args).await;
    let success = result.is_ok();
    if let Err(e) = audit.record(
        "create",
        "machine",
        &name,
        &format!("provider={}", provider),
        success,
    ) {
        error!("Failed to record audit entry: {}", e);
    }
    result.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn machine_delete(
    cli: State<'_, Arc<CliRunner>>,
    audit: State<'_, Arc<AuditLog>>,
    id: String,
    force: Option<bool>,
) -> Result<(), String> {
    let mut args = vec!["machine", "delete", &id];
    if force.unwrap_or(false) {
        args.push("--force");
    }
    let result = cli.run_raw(&args).await;
    let success = result.is_ok();
    let details = if force.unwrap_or(false) { "force=true" } else { "" };
    if let Err(e) = audit.record("delete", "machine", &id, details, success) {
        error!("Failed to record audit entry: {}", e);
    }
    result.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn machine_start(
    cli: State<'_, Arc<CliRunner>>,
    audit: State<'_, Arc<AuditLog>>,
    id: String,
) -> Result<(), String> {
    let result = cli.run_raw(&["machine", "start", &id]).await;
    let success = result.is_ok();
    if let Err(e) = audit.record("start", "machine", &id, "", success) {
        error!("Failed to record audit entry: {}", e);
    }
    result.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn machine_stop(
    cli: State<'_, Arc<CliRunner>>,
    audit: State<'_, Arc<AuditLog>>,
    id: String,
) -> Result<(), String> {
    let result = cli.run_raw(&["machine", "stop", &id]).await;
    let success = result.is_ok();
    if let Err(e) = audit.record("stop", "machine", &id, "", success) {
        error!("Failed to record audit entry: {}", e);
    }
    result.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn machine_status(
    cli: State<'_, Arc<CliRunner>>,
    id: String,
) -> Result<String, String> {
    cli.run_raw(&["machine", "status", &id, "--output", "json"])
        .await
        .map_err(|e| e.to_string())
}
