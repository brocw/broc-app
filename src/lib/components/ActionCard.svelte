<script lang="ts">
  import type { Action } from "$lib/actions";

  let { action }: { action: Action } = $props();
  let result = $state<string | null>(null);
  let loading = $state(false);

  async function run() {
    loading = true;
    result = null;
    try {
      result = await action.execute();
    } catch (e) {
      result = `Error: ${e}`;
    } finally {
      loading = false;
    }
  }
</script>

<button class="action-card" onclick={run} disabled={loading}>
  <span class="action-icon">{action.icon}</span>
  <span class="action-name">{action.name}</span>
  <span class="action-desc">{action.description}</span>
  {#if loading}
    <span class="action-result">Running...</span>
  {:else if result !== null}
    <span class="action-result">{result}</span>
  {/if}
</button>
