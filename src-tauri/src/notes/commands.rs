use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use rand::RngCore;
use uuid::Uuid;
use zeroize::Zeroize;

use super::types::{Note, NoteInfo, VaultFile, VaultKey, VaultState};
use super::{crypto, vault};

#[tauri::command]
pub fn notes_vault_exists(app: tauri::AppHandle) -> bool {
    vault::vault_exists(&app)
}

#[tauri::command]
pub fn notes_create_vault(
    app: tauri::AppHandle,
    state: tauri::State<'_, VaultState>,
    mut password: String,
) -> Result<(), String> {
    if vault::vault_exists(&app) {
        return Err("Vault already exists".to_string());
    }
    let mut salt_bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt_bytes);
    let key = crypto::derive_key(&password, &salt_bytes)?;
    password.zeroize();

    let notes: Vec<Note> = vec![];
    let plaintext = serde_json::to_vec(&notes).map_err(|e| e.to_string())?;
    let (ciphertext, nonce_bytes) = crypto::encrypt(&key, &plaintext)?;

    let vault_file = VaultFile {
        salt: BASE64.encode(salt_bytes),
        nonce: BASE64.encode(nonce_bytes),
        ciphertext: BASE64.encode(&ciphertext),
    };
    vault::write_vault(&app, &vault_file)?;
    *state.0.lock().map_err(|_| "Lock poisoned".to_string())? = Some(VaultKey(key));
    Ok(())
}

#[tauri::command]
pub fn notes_unlock(
    app: tauri::AppHandle,
    state: tauri::State<'_, VaultState>,
    mut password: String,
) -> Result<Vec<NoteInfo>, String> {
    let vault_file = vault::read_vault(&app).map_err(|_| "Invalid password".to_string())?;

    let salt_bytes: [u8; 16] = BASE64
        .decode(&vault_file.salt)
        .map_err(|_| "Invalid password".to_string())?
        .try_into()
        .map_err(|_| "Invalid password".to_string())?;

    let key = crypto::derive_key(&password, &salt_bytes).map_err(|_| "Invalid password".to_string())?;
    password.zeroize();

    let nonce_bytes: [u8; 12] = BASE64
        .decode(&vault_file.nonce)
        .map_err(|_| "Invalid password".to_string())?
        .try_into()
        .map_err(|_| "Invalid password".to_string())?;

    let ciphertext = BASE64
        .decode(&vault_file.ciphertext)
        .map_err(|_| "Invalid password".to_string())?;

    let plaintext = crypto::decrypt(&key, &nonce_bytes, &ciphertext)
        .map_err(|_| "Invalid password".to_string())?;

    let notes: Vec<Note> = serde_json::from_slice(&plaintext)
        .map_err(|_| "Invalid password".to_string())?;

    let note_infos: Vec<NoteInfo> = notes.iter().map(NoteInfo::from).collect();
    *state.0.lock().map_err(|_| "Lock poisoned".to_string())? = Some(VaultKey(key));
    Ok(note_infos)
}

#[tauri::command]
pub fn notes_lock(state: tauri::State<'_, VaultState>) {
    if let Ok(mut guard) = state.0.lock() {
        *guard = None;
    }
}

#[tauri::command]
pub fn notes_get_content(
    app: tauri::AppHandle,
    state: tauri::State<'_, VaultState>,
    note_id: String,
) -> Result<String, String> {
    let notes = read_notes_with_state(&app, &state)?;
    notes
        .iter()
        .find(|n| n.id == note_id)
        .map(|n| n.content.clone())
        .ok_or_else(|| "Note not found".to_string())
}

#[tauri::command]
pub fn notes_create(
    app: tauri::AppHandle,
    state: tauri::State<'_, VaultState>,
    title: String,
    description: String,
    content: String,
) -> Result<NoteInfo, String> {
    let mut notes = read_notes_with_state(&app, &state)?;
    let note = Note {
        id: Uuid::new_v4().to_string(),
        title,
        description,
        content,
    };
    let info = NoteInfo::from(&note);
    notes.push(note);
    write_notes_with_state(&app, &state, &notes)?;
    Ok(info)
}

#[tauri::command]
pub fn notes_update(
    app: tauri::AppHandle,
    state: tauri::State<'_, VaultState>,
    note_id: String,
    title: String,
    description: String,
    content: String,
) -> Result<NoteInfo, String> {
    let mut notes = read_notes_with_state(&app, &state)?;
    let note = notes
        .iter_mut()
        .find(|n| n.id == note_id)
        .ok_or_else(|| "Note not found".to_string())?;
    note.title = title;
    note.description = description;
    note.content = content;
    let info = NoteInfo::from(&*note);
    write_notes_with_state(&app, &state, &notes)?;
    Ok(info)
}

#[tauri::command]
pub fn notes_delete(
    app: tauri::AppHandle,
    state: tauri::State<'_, VaultState>,
    note_id: String,
) -> Result<(), String> {
    let mut notes = read_notes_with_state(&app, &state)?;
    let original_len = notes.len();
    notes.retain(|n| n.id != note_id);
    if notes.len() == original_len {
        return Err("Note not found".to_string());
    }
    write_notes_with_state(&app, &state, &notes)
}

// --- Private helpers ---

fn get_key(state: &tauri::State<'_, VaultState>) -> Result<[u8; 32], String> {
    state
        .0
        .lock()
        .map_err(|_| "Lock poisoned".to_string())?
        .as_ref()
        .map(|k| k.0)
        .ok_or_else(|| "Vault is locked".to_string())
}

fn read_notes_with_state(
    app: &tauri::AppHandle,
    state: &tauri::State<'_, VaultState>,
) -> Result<Vec<Note>, String> {
    let key = get_key(state)?;
    let vault_file = vault::read_vault(app)?;

    let nonce_bytes: [u8; 12] = BASE64
        .decode(&vault_file.nonce)
        .map_err(|_| "Vault corrupted".to_string())?
        .try_into()
        .map_err(|_| "Vault corrupted".to_string())?;

    let ciphertext = BASE64
        .decode(&vault_file.ciphertext)
        .map_err(|_| "Vault corrupted".to_string())?;

    let plaintext = crypto::decrypt(&key, &nonce_bytes, &ciphertext)?;
    serde_json::from_slice(&plaintext).map_err(|_| "Vault corrupted".to_string())
}

fn write_notes_with_state(
    app: &tauri::AppHandle,
    state: &tauri::State<'_, VaultState>,
    notes: &[Note],
) -> Result<(), String> {
    let key = get_key(state)?;
    let vault_file = vault::read_vault(app)?;
    let salt = vault_file.salt;

    let plaintext = serde_json::to_vec(notes).map_err(|e| e.to_string())?;
    let (ciphertext, nonce_bytes) = crypto::encrypt(&key, &plaintext)?;

    let new_vault = VaultFile {
        salt,
        nonce: BASE64.encode(nonce_bytes),
        ciphertext: BASE64.encode(&ciphertext),
    };
    vault::write_vault(app, &new_vault)
}
