<script>
  import Container from "$lib/Container.svelte";

  import Post from "$lib/Post.svelte";

  export let user;
  export let post;
  export let comments;
</script>

<Container>
  <Post {user} {post} {comments} detailed={true} />
</Container>
