<script>
  import SvelteMarkdown from "svelte-markdown";
  import Profile from "./auth/Profile.svelte";

  import Button from "./Button.svelte";
  import { timeAgo } from "./timeago";

  export let title = null;
  export let body = "";
  export let editable = null;
  export let clickable = false;

  export let user = null;
  export let created_at = null;

  let title_class =
    "font-bold font-mono bg-gray-800 text-light p-2 rounded-t-lg " +
    (clickable ? "hover:bg-green-500 hover:text-dark cursor-pointer" : "");

  const body_class =
    "font-serif bg-dark text-light p-2 flex-grow rounded-b-lg" +
    (title == null ? " rounded-t-lg" : "");
</script>

<div class="my-2 flex items-stretch flex-row">
  {#if editable}
    <form
      on:submit|preventDefault={(_) => editable(title, body)}
      class="m-2 flex flex-col flex-grow shadow-lg shadow-slate-600"
    >
      {#if title != null}
        <input class={title_class} bind:value={title} placeholder="Title" />
      {/if}
      <textarea
        class={body_class}
        type="text"
        placeholder="⋯"
        rows="16"
        bind:value={body}
      />
      <Button text="Submit" type="submit" />
    </form>
  {:else}
    <div class="m-2 flex flex-col flex-grow shadow-lg shadow-slate-600">
      {#if title != null}
        <h2 class={title_class} on:click>
          {title}
        </h2>
      {/if}

      <div class={body_class}>
        <div class="prose prose-invert prose-emerald prose-headings:sm">
          <SvelteMarkdown source={body} />
        </div>
      </div>
    </div>

    <div class="flex flex-col items-stretch">
      {#if user}
        <div class="bg-dark m-2 p-2 rounded-lg">
          <Profile {user} />
        </div>
      {/if}
      {#if created_at}
        <div class="bg-dark m-2 p-2 rounded-lg">
          <p class="text-light font-semibold text-center">
            {timeAgo(new Date(created_at + "Z"))}
          </p>
        </div>
      {/if}
    </div>
  {/if}
</div>
