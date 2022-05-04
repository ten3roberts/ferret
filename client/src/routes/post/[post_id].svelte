<script>
  import Container from "$lib/Container.svelte";

  import Post from "$lib/Post.svelte";

  export let user;
  export let post;
  export let comments;
  export let solved_by;
</script>

<Container>
  <Post {user} {post} {comments} {solved_by} detailed={true} />
</Container>
