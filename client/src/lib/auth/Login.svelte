<script>
  import auth from "./service";
  import { isAuthenticated, user } from "../stores";

  import Profile from "./Profile.svelte";
</script>

<div>
  {#if $isAuthenticated}
    <Profile user={$user} />
  {:else}
    <button
      class="bg-emerald-400 py-2 px-3 hover:bg-fuchsia-900"
      on:click={auth.login}>Login</button
    >
  {/if}
</div>
