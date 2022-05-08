<script>
  import Posts from "$lib/Posts.svelte";
  export let posts;
  console.log("Got: " + posts);
</script>

<Posts {posts} />

<h2 class="text-lg text-center">Found {posts.length} matches</h2>
