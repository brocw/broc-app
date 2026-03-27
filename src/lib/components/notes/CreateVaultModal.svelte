<script lang="ts">
  import { createVault, closeModal } from "$lib/notes/store.svelte";

  let password = $state("");
  let confirm = $state("");
  let error = $state<string | null>(null);
  let loading = $state(false);

  const mismatch = $derived(confirm.length > 0 && password !== confirm);
  const canSubmit = $derived(password.length >= 8 && password === confirm);

  async function submit() {
    if (!canSubmit) return;
    loading = true;
    error = null;
    let succeeded = false;
    try {
      await createVault(password);
      succeeded = true;
    } catch (e) {
      error = "Failed to create vault. Please try again.";
    } finally {
      loading = false;
      password = "";
      confirm = "";
    }
    if (succeeded) closeModal();
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "Enter") submit();
    if (e.key === "Escape") closeModal();
  }
</script>

<div class="modal-overlay" role="presentation" onmousedown={closeModal}>
  <div
    class="modal"
    role="dialog"
    aria-modal="true"
    aria-labelledby="create-vault-title"
    tabindex="-1"
    onmousedown={(e) => e.stopPropagation()}
  >
    <h2 id="create-vault-title">Create Notes Vault</h2>
    <p class="modal-subtitle">
      Choose a master password to encrypt your notes. This cannot be recovered if lost.
    </p>

    <input
      type="password"
      placeholder="Master password (min. 8 characters)"
      bind:value={password}
      {onkeydown}
      disabled={loading}
      autocomplete="new-password"
    />
    <input
      type="password"
      placeholder="Confirm password"
      bind:value={confirm}
      {onkeydown}
      disabled={loading}
      autocomplete="new-password"
    />

    {#if mismatch}
      <p class="modal-error">Passwords do not match.</p>
    {/if}
    {#if error}
      <p class="modal-error">{error}</p>
    {/if}

    <div class="modal-actions">
      <button class="btn-secondary" onclick={closeModal} disabled={loading}>Cancel</button>
      <button class="btn-primary" onclick={submit} disabled={loading || !canSubmit}>
        {loading ? "Creating…" : "Create Vault"}
      </button>
    </div>
  </div>
</div>
