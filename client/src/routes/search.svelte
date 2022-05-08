<script>
  import Posts from "$lib/Posts.svelte";
  export let posts;
  console.log("Got: " + posts);
</script>

<Posts {posts} />
