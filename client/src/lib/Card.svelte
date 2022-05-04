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
    "font-bold font-mono text-light p-2 rounded-lg " +
    (clickable ? "hover:bg-green-500 hover:text-dark cursor-pointer" : "");

  const body_class = "text-light p-2 flex-grow";
</script>

<div
  class="my-5 flex items-stretch flex-row shadow-md shadow-gray-900 bg-dark
  rounded-lg"
>
  {#if editable}
    <form
      on:submit|preventDefault={(_) => editable(title, body)}
      class="m-2 flex flex-col flex-grow"
    >
      {#if title != null}
        <input
          class={title_class + " bg-gray-800"}
          bind:value={title}
          placeholder="Title"
        />
      {/if}
      <textarea
        class={body_class + " bg-dark"}
        type="text"
        placeholder="⋯"
        rows="8"
        bind:value={body}
      />
      <Button text="Submit" type="submit" />
    </form>
  {:else}
    <div class="m-2 flex flex-col flex-grow justify-end">
      {#if title != null}
        <h2 class={title_class} on:click>
          {title}
        </h2>
      {/if}
      <div class={body_class}>
        <SvelteMarkdown source={body} />
        <div class="prose prose-invert prose-blue prose-headings:sm" />
      </div>
      <slot />
    </div>

    <div class="flex flex-col items-stretch rounded-lg m-2 p-2 bg-gray-800">
      {#if user}
        <Profile {user} />
      {/if}
      {#if created_at}
        <p class="text-light font-semibold text-center">
          {timeAgo(new Date(created_at + "Z"))}
        </p>
      {/if}
    </div>
  {/if}
</div>
