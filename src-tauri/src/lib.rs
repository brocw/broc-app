mod actions;
mod notes;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(notes::VaultState(std::sync::Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            actions::hello::greet,
            actions::sysinfo::get_sysinfo,
            notes::commands::notes_vault_exists,
            notes::commands::notes_create_vault,
            notes::commands::notes_unlock,
            notes::commands::notes_lock,
            notes::commands::notes_get_content,
            notes::commands::notes_create,
            notes::commands::notes_update,
            notes::commands::notes_delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
