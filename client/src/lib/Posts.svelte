<script>
  export let posts;
  console.log(posts);
  import Post from "./Post.svelte";
  import Container from "./Container.svelte";
  console.log(posts);
</script>

<Container>
  {#each posts as post}
    <Post {...post} />
  {/each}
</Container>
