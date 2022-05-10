<script>
  import Posts from "$lib/Posts.svelte";
  export let posts;
  console.log("Got: " + posts);
  import { searchQuery } from "$lib/stores";
  let page = 0;
  let search = $searchQuery;
  let params = new URLSearchParams({
    searchQuery: search,
    page: page.toString(),
  });
</script>

<Posts {posts} />

<h2 class="text-lg text-center">Found {posts.length} matches</h2>
