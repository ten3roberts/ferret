<script>
  import Button from "./Button.svelte";

  export let title = "";
  export let body = "";
  export let editable = null;

  const title_class =
    "w-full font-bold font-mono bg-gray-800 text-light p-2 rounded-t-lg";
  const body_class =
    "h-3/5 w-full font-serif bg-dark text-light p-2 rounded-b-lg h-32";
</script>

<div class="m-5 rounded-lg shadow-slate-600 shadow-sm">
  {#if editable}
    <form
      on:submit|preventDefault={(_) => editable(title, body)}
      class="flex items-center flex-col"
    >
      <input class={title_class} bind:value={title} placeholder="Title" />
      <textarea
        class={body_class}
        type="text"
        placeholder="⋯"
        bind:value={body}
      />
      <Button text="Submit" type="submit" />
    </form>
  {:else}
    {#if title != ""}
      <h2 class={title_class}>
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
