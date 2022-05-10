<script>
  /// Represents a list of non detailed posts
  export let posts;
  export let more = null;

  import Post from "./Post.svelte";
  import Container from "./Container.svelte";
  import Button from "./Button.svelte";
</script>

<Container>
  {#each posts as post}
    <Post {...post} />
  {/each}
  {#if more != null}
    <Button text="More" on:click={more} />
  {/if}
</Container>
