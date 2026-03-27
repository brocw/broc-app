<script lang="ts">
  import { unlock, closeModal } from "$lib/notes/store.svelte";

  let password = $state("");
  let error = $state<string | null>(null);
  let loading = $state(false);

  async function submit() {
    if (!password) return;
    loading = true;
    error = null;
    let succeeded = false;
    try {
      await unlock(password);
      succeeded = true;
    } catch {
      error = "Incorrect password.";
    } finally {
      loading = false;
      password = "";
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
    aria-labelledby="unlock-title"
    tabindex="-1"
    onmousedown={(e) => e.stopPropagation()}
  >
    <h2 id="unlock-title">Unlock Notes</h2>
    <p class="modal-subtitle">Enter your master password to unlock your notes.</p>

    <input
      type="password"
      placeholder="Master password"
      bind:value={password}
      {onkeydown}
      disabled={loading}
      autocomplete="current-password"
    />

    {#if error}
      <p class="modal-error">{error}</p>
    {/if}

    <div class="modal-actions">
      <button class="btn-secondary" onclick={closeModal} disabled={loading}>Cancel</button>
      <button class="btn-primary" onclick={submit} disabled={loading || !password}>
        {loading ? "Unlocking…" : "Unlock"}
      </button>
    </div>
  </div>
</div>
