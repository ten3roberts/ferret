<script>
  export let posts;
  import Post from "./Post.svelte";
</script>

<div class="w-3/5 m-auto shadow-lg shadow-fuchsia-900 h-screen">
  {#each posts as post}
    <Post {...post} />
    <!-- content here -->
  {/each}
</div>
