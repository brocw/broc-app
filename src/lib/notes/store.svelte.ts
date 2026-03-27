import { invoke } from "@tauri-apps/api/core";

export type NoteInfo = { id: string; title: string; description: string };

export type ActiveModal = "unlock" | "createVault" | "editNote" | null;

export const notesState = $state({
  vaultExists: null as boolean | null,
  isUnlocked: false,
  notes: [] as NoteInfo[],
  activeModal: null as ActiveModal,
  editingNote: null as NoteInfo | null,
});

export function openModal(modal: ActiveModal, note: NoteInfo | null = null): void {
  notesState.activeModal = modal;
  notesState.editingNote = note;
}

export function closeModal(): void {
  notesState.activeModal = null;
  notesState.editingNote = null;
}

export async function checkVault(): Promise<void> {
  notesState.vaultExists = await invoke<boolean>("notes_vault_exists");
}

export async function createVault(password: string): Promise<void> {
  await invoke("notes_create_vault", { password });
  notesState.vaultExists = true;
  notesState.isUnlocked = true;
  notesState.notes = [];
}

export async function unlock(password: string): Promise<void> {
  const result = await invoke<NoteInfo[]>("notes_unlock", { password });
  notesState.notes = result;
  notesState.isUnlocked = true;
}

export async function lock(): Promise<void> {
  await invoke("notes_lock");
  notesState.isUnlocked = false;
  notesState.notes = [];
}

export async function createNote(
  title: string,
  description: string,
  content: string
): Promise<NoteInfo> {
  const info = await invoke<NoteInfo>("notes_create", { title, description, content });
  notesState.notes = [...notesState.notes, info];
  return info;
}

export async function updateNote(
  noteId: string,
  title: string,
  description: string,
  content: string
): Promise<void> {
  const updated = await invoke<NoteInfo>("notes_update", {
    noteId,
    title,
    description,
    content,
  });
  notesState.notes = notesState.notes.map((n) => (n.id === noteId ? updated : n));
}

export async function deleteNote(noteId: string): Promise<void> {
  await invoke("notes_delete", { noteId });
  notesState.notes = notesState.notes.filter((n) => n.id !== noteId);
}
