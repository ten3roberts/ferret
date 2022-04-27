<script>
  /// Represents a post card
  import Profile from "./auth/Profile.svelte";
  import { timeAgo } from "$lib/timeago";
  import Card from "./Card.svelte";
  import Button from "./Button.svelte";
  import CreateComment from "./CreateComment.svelte";

  export let post;
  export let user;
  export let comments = [];
  export let detailed = false;
  let replying = false;
</script>

<div class="flex flex-col">
  <Card
    title={post.title}
    body={post.body}
    clickable={true}
    {user}
    created_at={post.created_at}
    on:click={() => (location.href = "/post/" + post.post_id)}
  />

  {#if detailed}
    <hr class="border-dashed m-5 border-gray-600" />

    <div class="mx-10">
      <!-- content here -->
      {#each comments as { comment, user }}
        <Card
          title={comment.title}
          body={comment.body}
          {user}
          created_at={comment.created_at}
        />
      {/each}

      {#if replying}
        <Button text="Discard" on:click={(_) => (replying = false)} />
        <CreateComment post_id={post.post_id} />
      {:else}
        <Button text="Reply" on:click={(_) => (replying = true)} />
      {/if}
    </div>
  {/if}
</div>
