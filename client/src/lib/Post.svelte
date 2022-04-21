<script>
  /// Represents a post card
  import Profile from "./auth/Profile.svelte";
  import Card from "./Card.svelte";

  export let post;
  export let user;
</script>

<div class="flex flex-grow flex-row items-stretch p-5">
  <Card
    title={post.title}
    body={post.body}
    clickable={true}
    on:click={() => (location.href = "/post/" + post.id)}
  />
  <div class="bg-dark m-2 p-2 w-1/5 rounded-lg">
    <Profile {user} />
    {#if post.created_at}
      <h2 class="text-light text-center">
        {new Date(post.created_at).toLocaleString()}
      </h2>
    {/if}
  </div>
</div>
