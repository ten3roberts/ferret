<script>
  import { page } from "$app/stores";

  import Button from "./Button.svelte";
  import { searchQuery } from "./stores";

  searchQuery.set($page.url.searchParams.get("searchQuery"));
  function search() {
    console.log("Searching");
    let search = $searchQuery;
    const params = new URLSearchParams({
      searchQuery: search,
    });

    const url = `/search?${params}`;
    console.log(url);
    location.href = url;
  }
</script>

<form on:submit|preventDefault={(_) => search()}>
  <input
    class="bg-dark text-light rounded-md p-2"
    placeholder="Search"
    bind:value={$searchQuery}
  />
  <Button text="Search" type="submit" />
</form>
