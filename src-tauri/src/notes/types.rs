use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub description: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NoteInfo {
    pub id: String,
    pub title: String,
    pub description: String,
}

impl From<&Note> for NoteInfo {
    fn from(note: &Note) -> Self {
        NoteInfo {
            id: note.id.clone(),
            title: note.title.clone(),
            description: note.description.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct VaultFile {
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct VaultKey(pub [u8; 32]);

pub struct VaultState(pub std::sync::Mutex<Option<VaultKey>>);
