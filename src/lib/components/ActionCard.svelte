<script lang="ts">
  import type { Action } from "$lib/actions";

  let { action }: { action: Action } = $props();
  let result = $state<string | null>(null);
  let loading = $state(false);

  $effect(() => {
    action;
    result = null;
  });

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

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      if (!loading) run();
    }
  }
</script>

<div
  class="action-card"
  class:disabled={loading}
  role="button"
  tabindex="0"
  onclick={run}
  onkeydown={onkeydown}
  aria-disabled={loading}
>
  <span class="action-icon">{action.icon}</span>
  <span class="action-name">{action.name}</span>
  <span class="action-desc">{action.description}</span>
  {#if loading}
    <span class="action-result">Running...</span>
  {:else if result !== null}
    <span class="action-result">{result}</span>
  {/if}
  {#if action.secondaryAction}
    <button
      class="action-secondary-btn"
      onclick={(e) => { e.stopPropagation(); action.secondaryAction!.execute(); }}
    >
      {action.secondaryAction.label}
    </button>
  {/if}
</div>
