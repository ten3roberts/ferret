<script>
  import Header from "../lib/Header.svelte";
  import Posts from "../lib/Posts.svelte";
  import { posts } from "../lib/stores.js";
</script>

<Header />
<Posts posts={$posts} />
