<script lang="ts">
  import { onMount } from "svelte";
  import "$lib/actions";
  import SearchBar from "$lib/components/SearchBar.svelte";
  import ActionGrid from "$lib/components/ActionGrid.svelte";
  import VaultCard from "$lib/components/notes/VaultCard.svelte";
  import UnlockModal from "$lib/components/notes/UnlockModal.svelte";
  import CreateVaultModal from "$lib/components/notes/CreateVaultModal.svelte";
  import NoteEditModal from "$lib/components/notes/NoteEditModal.svelte";
  import { checkVault, notesState } from "$lib/notes/store.svelte";
  import { registerNoteActions, unregisterNoteActions } from "$lib/actions/notes";

  let query = $state("");

  onMount(() => {
    checkVault();
  });

  let prevNoteIds: string[] = [];

  $effect(() => {
    if (notesState.isUnlocked) {
      unregisterNoteActions(prevNoteIds.filter((id) => !notesState.notes.some((n) => n.id === id)));
      registerNoteActions(notesState.notes);
      prevNoteIds = notesState.notes.map((n) => n.id);
    } else {
      unregisterNoteActions(prevNoteIds);
      prevNoteIds = [];
    }
  });
</script>

<main class="dashboard">
  <h1>Broc App</h1>
  <SearchBar bind:query />

  <div class="vault-row">
    <VaultCard />
  </div>

  <ActionGrid {query} />
</main>

{#if notesState.activeModal === "unlock"}
  <UnlockModal />
{/if}

{#if notesState.activeModal === "createVault"}
  <CreateVaultModal />
{/if}

{#if notesState.activeModal === "editNote"}
  <NoteEditModal />
{/if}
