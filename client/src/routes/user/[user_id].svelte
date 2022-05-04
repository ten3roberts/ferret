<script>
  import Profile from "$lib/auth/Profile.svelte";
  import Container from "$lib/Container.svelte";
  import Post from "$lib/Post.svelte";

  export let user;
  export let posts;
</script>

<Container>
  <div class="bg-gray-800 rounded-lg p-5">
    <Profile {user} />
  </div>

  {#each posts as post}
    <Post {post} {user} show_user={false} />
  {/each}
</Container>
