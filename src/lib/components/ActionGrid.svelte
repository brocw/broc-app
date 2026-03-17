<script lang="ts">
  import { searchActions } from "$lib/actions";
  import ActionCard from "./ActionCard.svelte";

  let { query = "" }: { query: string } = $props();
  let filtered = $derived(searchActions(query));
</script>

<div class="action-grid">
  {#each filtered as action (action.id)}
    <ActionCard {action} />
  {/each}
  {#if filtered.length === 0}
    <p class="no-results">No actions found.</p>
  {/if}
</div>
