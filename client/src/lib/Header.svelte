<script>
  import Login from "./auth/Login.svelte";

  const routes = ["About", "Home"];
</script>

<nav class="shadow flex justify-between items-center">
  <ol class="flex justify-start items-center">
    <h1 class="font-bold px-5">Ferret</h1>
    {#each routes as route}
      <li class="py-2 px-3 hover:bg-emerald-400">
        <a href={"/" + route.toLowerCase()}>{route}</a>
      </li>
    {/each}
  </ol>
  <Login />
</nav>
