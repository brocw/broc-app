<script lang="ts">
  import { untrack } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { createNote, updateNote, deleteNote, closeModal, notesState } from "$lib/notes/store.svelte";

  // Snapshot prop once — modal is re-created on each open.
  const _note = untrack(() => notesState.editingNote);
  const isEditing = _note !== null;

  let title = $state(_note?.title ?? "");
  let description = $state(_note?.description ?? "");
  let content = $state("");
  let loading = $state(isEditing);
  let saving = $state(false);
  let confirming = $state(false);
  let error = $state<string | null>(null);

  const canSave = $derived(title.trim().length > 0 && !loading && !saving);

  $effect(() => {
    if (isEditing && _note) {
      invoke<string>("notes_get_content", { noteId: _note.id })
        .then((c) => {
          content = c;
          loading = false;
        })
        .catch(() => {
          error = "Failed to load note content.";
          loading = false;
        });
    }
  });

  async function save() {
    if (!canSave) return;
    saving = true;
    error = null;
    let succeeded = false;
    try {
      if (isEditing && _note) {
        await updateNote(_note.id, title.trim(), description.trim(), content);
      } else {
        await createNote(title.trim(), description.trim(), content);
      }
      succeeded = true;
    } catch {
      error = "Failed to save note.";
    } finally {
      saving = false;
    }
    if (succeeded) closeModal();
  }

  async function confirmDelete() {
    if (!_note) return;
    saving = true;
    error = null;
    let succeeded = false;
    try {
      await deleteNote(_note.id);
      succeeded = true;
    } catch {
      error = "Failed to delete note.";
      saving = false;
    }
    if (succeeded) closeModal();
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "Escape") closeModal();
  }
</script>

<div class="modal-overlay" role="presentation" onmousedown={closeModal}>
  <div
    class="modal modal-large"
    role="dialog"
    aria-modal="true"
    aria-labelledby="note-edit-title"
    tabindex="-1"
    onmousedown={(e) => e.stopPropagation()}
  >
    <h2 id="note-edit-title">{isEditing ? "Edit Note" : "New Note"}</h2>

    {#if loading}
      <p class="modal-subtitle">Loading…</p>
    {:else}
      <input
        type="text"
        placeholder="Title"
        bind:value={title}
        {onkeydown}
        disabled={saving}
      />
      <input
        type="text"
        placeholder="Description (shown in action grid)"
        bind:value={description}
        {onkeydown}
        disabled={saving}
      />
      <textarea
        placeholder="Note content"
        bind:value={content}
        disabled={saving}
        rows="8"
      ></textarea>

      {#if error}
        <p class="modal-error">{error}</p>
      {/if}

      {#if confirming}
        <p class="modal-confirm-text">Delete this note? This cannot be undone.</p>
      {/if}

      <div class="modal-actions">
        {#if isEditing}
          {#if confirming}
            <button class="btn-danger" onclick={confirmDelete} disabled={saving}>
              {saving ? "Deleting…" : "Confirm Delete"}
            </button>
            <button class="btn-secondary" onclick={() => (confirming = false)} disabled={saving}>
              Cancel
            </button>
          {:else}
            <button class="btn-danger-outline" onclick={() => (confirming = true)} disabled={saving}>
              Delete
            </button>
          {/if}
        {/if}
        <div class="modal-actions-right">
          <button class="btn-secondary" onclick={closeModal} disabled={saving}>Cancel</button>
          <button class="btn-primary" onclick={save} disabled={!canSave || saving}>
            {saving ? "Saving…" : "Save"}
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>
