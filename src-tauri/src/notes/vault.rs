use std::path::PathBuf;

use tauri::Manager;

use super::types::VaultFile;

pub fn vault_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Could not resolve app data dir: {}", e))?;
    Ok(dir.join("notes.vault"))
}

pub fn vault_exists(app: &tauri::AppHandle) -> bool {
    vault_path(app).map(|p| p.exists()).unwrap_or(false)
}

pub fn read_vault(app: &tauri::AppHandle) -> Result<VaultFile, String> {
    let path = vault_path(app)?;
    let contents =
        std::fs::read_to_string(&path).map_err(|e| format!("Failed to read vault: {}", e))?;
    serde_json::from_str(&contents).map_err(|_| "Vault file appears corrupted".to_string())
}

pub fn write_vault(app: &tauri::AppHandle, vault: &VaultFile) -> Result<(), String> {
    let path = vault_path(app)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }
    let tmp_path = path.with_extension("vault.tmp");
    let contents =
        serde_json::to_string(vault).map_err(|e| format!("Serialization error: {}", e))?;
    std::fs::write(&tmp_path, &contents)
        .map_err(|e| format!("Failed to write vault: {}", e))?;
    std::fs::rename(&tmp_path, &path)
        .map_err(|e| format!("Failed to finalize vault write: {}", e))?;
    Ok(())
}
