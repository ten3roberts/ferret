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

<div class="flex flex-col justify-center p-5">
  <div class="flex flex-grow flex-row items-stretch">
    <Card
      title={post.title}
      body={post.body}
      clickable={true}
      on:click={() => (location.href = "/post/" + post.post_id)}
    />
    <div class="bg-dark m-2 p-2 w-1/5 rounded-lg">
      <Profile {user} />
      {#if post.created_at}
        <h2 class="text-light text-center">
          {timeAgo(new Date(post.created_at + "Z"))}
        </h2>
      {/if}
    </div>
  </div>

  {#if detailed}
    <!-- content here -->
    <div class="p-5">
      {#each comments as { comment, user }}
        <div class="flex flex-grow flex-row items-stretch">
          <Card title={comment.title} body={comment.body} />
          <div class="bg-dark m-2 p-2 w-1/5 rounded-lg">
            <Profile {user} />
            <h2 class="text-light text-center">
              {timeAgo(new Date(comment.created_at + "Z"))}
            </h2>
          </div>
        </div>
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
