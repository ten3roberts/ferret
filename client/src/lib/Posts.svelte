<script>
  export let posts;
  console.log(posts);
  import Post from "./Post.svelte";
</script>

<div class="w-3/5 m-auto shadow-md shadow-fuchsia-900 h-screen">
  {#each posts as post}
    <Post {...post} />
    <!-- content here -->
  {/each}
</div>
