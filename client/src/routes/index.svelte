<script>
  import Posts from "../lib/Posts.svelte";
  export let posts;
  console.log("Posts: " + posts);
</script>

<Posts {posts} />
