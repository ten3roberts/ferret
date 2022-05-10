<script>
  import Posts from "../lib/Posts.svelte";
  let page = 0;
  async function getPosts() {
    return fetch("/api/posts?page=" + page, { method: "GET" })
      .then((v) => v.json())
      .then((v) => {
        return v;
      });
  }
  let result = getPosts();
</script>

{#await result}
  <h3>Loading</h3>
{:then value}
  <Posts
    posts={value}
    more={() => {
      page += 1;
      /* result = new Promise(async () => [...value, await getPosts()]); */
      getPosts().then((page) => {
        value = [...value, ...page];
      });
    }}
  />
{:catch error}
  <p style="color: red">{error}</p>
{/await}
