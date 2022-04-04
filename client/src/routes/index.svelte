<script>
  import Button from "$lib/Button.svelte";
  import Post from "$lib/Post.svelte";
  import Posts from "../lib/Posts.svelte";
  async function getPosts() {
    return fetch("/api/posts", { method: "GET" }).then((v) => v.json());
  }
  let result = getPosts();
</script>

{#await result}
  <h3>Loading</h3>
{:then value}
  <Posts posts={value} />
{:catch error}
  <p style="color: red">{error}</p>
{/await}
