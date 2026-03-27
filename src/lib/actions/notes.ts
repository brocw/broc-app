import { invoke } from "@tauri-apps/api/core";
import { registerAction, unregisterAction } from "./registry";
import type { NoteInfo } from "$lib/notes/store.svelte";

export function registerNoteActions(noteList: NoteInfo[]): void {
  for (const note of noteList) {
    registerAction({
      id: `note:${note.id}`,
      name: note.title,
      description: note.description,
      icon: "📝",
      category: "Notes",
      execute: async () => {
        const content = await invoke<string>("notes_get_content", { noteId: note.id });
        return content;
      },
    });
  }
}

export function unregisterNoteActions(ids: string[]): void {
  for (const id of ids) {
    unregisterAction(`note:${id}`);
  }
}
