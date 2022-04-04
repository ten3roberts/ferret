<script>
  import Card from "./Card.svelte";

  export let title;
  export let body;
  export let username;
  export let id;
  export let created_at;
</script>

<div class="flex flex-grow flex-row items-stretch">
  <Card {title} {body} on:click={() => (location.href = "/post/" + id)} />
  <div class="bg-dark m-5 p-2 w-1/5 rounded-lg">
    <h1 class="text-light font-bold">{username}</h1>
    {#if created_at}
      <h2 class="text-light">
        Created: {new Date(created_at).toLocaleDateString()}
      </h2>
    {/if}
  </div>
</div>
