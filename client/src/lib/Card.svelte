<script>
  import Button from "./Button.svelte";

  export let title = null;
  export let body = "";
  export let editable = null;
  export let clickable = false;

  let title_class =
    "w-full font-bold font-mono bg-gray-800 text-light p-2 rounded-t-lg " +
    (clickable ? "hover:bg-green-500 hover:text-dark cursor-pointer" : "");

  const body_class =
    "h-3/5 w-full font-serif bg-dark text-light p-2 rounded-b-lg h-32" +
    (title == null ? " rounded-t-lg" : "");
</script>

<div class="my-2 w-full rounded-lg shadow-slate-600 shadow-sm">
  {#if editable}
    <form
      on:submit|preventDefault={(_) => editable(title, body)}
      class="flex items-center flex-col"
    >
      {#if title != null}
        <input class={title_class} bind:value={title} placeholder="Title" />
      {/if}
      <textarea
        class={body_class}
        type="text"
        placeholder="⋯"
        bind:value={body}
      />
      <Button text="Submit" type="submit" />
    </form>
  {:else}
    {#if title != null}
      <h2 class={title_class} on:click>
        {title}
      </h2>
    {/if}

    <div class={body_class}>
      {#each body.split("\n\n") as p}
        <p>{p}</p>
      {/each}
    </div>
  {/if}
</div>
