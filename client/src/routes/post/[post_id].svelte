<script>
  import Container from "$lib/Container.svelte";

  import Post from "$lib/Post.svelte";

  export let user;
  export let post;
</script>

<Container>
  <Post {user} {post} />
</Container>
