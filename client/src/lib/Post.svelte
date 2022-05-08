<script>
  /// Represents a post card
  import Profile from "./auth/Profile.svelte";
  import { timeAgo } from "$lib/timeago";
  import Card from "./Card.svelte";
  import { isAuthenticated, user as cur_user } from "./stores";
  import Button from "./Button.svelte";
  import CreateComment from "./CreateComment.svelte";
  import auth from "./auth/service";
  import Container from "./Container.svelte";
  import { stringify } from "postcss";

  export let post;
  export let user;
  export let solved_by = null;
  export let show_user = true;
  export let comments = [];
  export let detailed = false;
  let replying = false;

  async function mark_solved(comment) {
    console.log("Marking as solved");
    let token = await auth.getToken();
    solved_by = comment;
    fetch("/api/posts", {
      method: "PATCH",
      body: JSON.stringify({
        post_id: post.post_id,
        comment_id: comment,
        token,
      }),
    }).then((v) => {
      if (v.redirected) {
        location.href = v.url;
      }
    });
  }

  async function del_post(id) {
    console.log("Deleting post");
    let token = await auth.getToken();

    await fetch("/api/posts", {
      method: "DELETE",
      body: JSON.stringify({ id, token }),
    });

    location.href = "/";
  }

  async function del_comment(id) {
    const response = await fetch("/api/comments", {
      method: "DELETE",
      body: JSON.stringify({ id, token: await auth.getToken() }),
    });
    if (response.ok) {
      comments = comments.filter((v) => v.comment.comment_id != id);
    }
  }
  $: owner = $isAuthenticated && $cur_user.sub == user.user_id;
</script>

<div class="flex flex-col">
  <Card
    title={post.title}
    body={post.body}
    clickable={true}
    user={show_user ? user : null}
    created_at={post.created_at}
    on:click={() => (location.href = "/post/" + post.post_id)}
  >
    {#if detailed && owner == true}
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
          user={show_user ? user : null}
          created_at={comment.created_at}
          background={solved_by == comment.comment_id
            ? "bg-green-500"
            : "bg-dark"}
        >
          {#if $isAuthenticated && $cur_user.sub == user.user_id}
            <div class="flex flex-col justify-start">
              <Button
                text="Delete"
                on:click={(_) => del_comment(comment.comment_id)}
              />
            </div>
          {/if}
          {#if detailed && owner}
            <Button
              text="Mark Solved"
              on:click={(_) => mark_solved(comment.comment_id)}
            />
          {/if}
        </Card>
      {/each}

      {#if replying}
        <Button text="Discard" on:click={(_) => (replying = false)} />
        <CreateComment
          post_id={post.post_id}
          on_create={(v) => {
            replying = false;
            comments = [...comments, v];
          }}
        />
      {:else}
        <Button text="Reply" on:click={(_) => (replying = true)} />
      {/if}
    </div>
  {/if}
</div>
