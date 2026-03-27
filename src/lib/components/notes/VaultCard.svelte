<script lang="ts">
  import { notesState, lock, openModal } from "$lib/notes/store.svelte";
</script>

<div class="vault-card" class:unlocked={notesState.isUnlocked}>
  <span class="vault-icon">{notesState.isUnlocked ? "🔓" : "🔒"}</span>
  <span class="vault-name">Notes Vault</span>

  {#if notesState.vaultExists === null}
    <span class="vault-status">Loading…</span>
  {:else if !notesState.isUnlocked}
    <span class="vault-status">{notesState.vaultExists ? "Locked" : "No vault"}</span>
    <button
      class="vault-btn"
      onclick={() => openModal(notesState.vaultExists ? "unlock" : "createVault")}
    >
      {notesState.vaultExists ? "Unlock" : "Create Vault"}
    </button>
  {:else}
    <span class="vault-status">Unlocked</span>
    <div class="vault-controls">
      <button class="vault-btn" onclick={() => openModal("editNote", null)}>+ Note</button>
      <button class="vault-btn vault-btn-lock" onclick={lock}>Lock</button>
    </div>
  {/if}
</div>
