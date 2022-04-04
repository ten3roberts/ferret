<script>
  import Container from "$lib/Container.svelte";

  import Post from "$lib/Post.svelte";

  export let title = "";
  export let body = "";
  export let id = 0;
</script>

<Container>
  <Post {title} {body} {id} />
</Container>
