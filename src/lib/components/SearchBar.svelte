<script lang="ts">
  let { query = $bindable("") }: { query: string } = $props();
</script>

<div class="search-bar">
  <input
    type="text"
    placeholder="Search actions..."
    bind:value={query}
  />
</div>
