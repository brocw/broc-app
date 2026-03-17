<script lang="ts">
  import "$lib/actions";
  import SearchBar from "$lib/components/SearchBar.svelte";
  import ActionGrid from "$lib/components/ActionGrid.svelte";

  let query = $state("");
</script>

<main class="dashboard">
  <h1>Broc App</h1>
  <SearchBar bind:query />
  <ActionGrid {query} />
</main>
