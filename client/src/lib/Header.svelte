<script>
  import Login from "./auth/Login.svelte";
  import Button from "./Button.svelte";

  const routes = ["About", "Home"];
</script>

<nav
  class="shadow-lg flex justify-between items-center text-light bg-gray-800 sticky top-0
  w-full"
>
  <ol class="flex justify-start items-center">
    <a href="/" class="font-bold px-5">Ferret</a>
    {#each routes as route}
      <li class="py-2 px-3 hover:bg-green">
        <a href={"/" + route.toLowerCase()}>{route}</a>
      </li>
    {/each}
  </ol>

  <Button
    on:click={(_) => (location.href = "/create_post")}
    text="Create Post"
  />
  <Login />
</nav>
