<script>
  import Header from "../lib/Header.svelte";
  import Posts from "../lib/Posts.svelte";
  export let posts;
  console.log("Posts: " + posts);
</script>

<Header />
<Posts {posts} />
