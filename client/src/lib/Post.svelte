<script>
  /// Represents a post card
  import Profile from "./auth/Profile.svelte";
  import { timeAgo } from "$lib/timeago";
  import Card from "./Card.svelte";
  import { isAuthenticated, user as cur_user } from "./stores";
  import Button from "./Button.svelte";
  import CreateComment from "./CreateComment.svelte";
  import auth from "./auth/service";

  export let post;
  export let user;
  export let comments = [];
  export let detailed = false;
  let replying = false;

  /* $: ({ user_id } = $cur_user); */
  /* $: console.log("User: " + user_id); */

  async function del_post(id) {
    console.log("Deleting post");
    let token = await auth.getToken();
    console.log("Got token");
    fetch("/api/posts", {
      method: "DELETE",
      body: JSON.stringify({ id, token }),
    }).then((v) => {
      console.log("got: " + v.ok);
      if (v.redirected) {
        location.href = v.url;
      }
    });
  }

  async function del_comment(id) {
    fetch("/api/comments", {
      method: "DELETE",
      body: JSON.stringify({ id, token: await auth.getToken() }),
    }).then((v) => {
      if (v.redirected) {
        location.href = v.url;
      }
    });
  }
</script>

<div class="flex flex-col">
  <Card
    title={post.title}
    body={post.body}
    clickable={true}
    {user}
    created_at={post.created_at}
    on:click={() => (location.href = "/post/" + post.post_id)}
  >
    {#if $isAuthenticated && $cur_user.sub == user.user_id}
      <div class="flex flex-col justify-start">
        <Button text="Delete" on:click={(_) => del_post(post.post_id)} />
      </div>
    {/if}
  </Card>

  {#if detailed}
    <div>
      <!-- content here -->
      {#each comments as { comment, user }}
        <Card
          title={comment.title}
          body={comment.body}
          {user}
          created_at={comment.created_at}
        >
          {#if $isAuthenticated && $cur_user.sub == user.user_id}
            <div class="flex flex-col justify-start">
              <Button
                text="Delete"
                on:click={(_) => del_comment(comment.comment_id)}
              />
            </div>
          {/if}
        </Card>
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
