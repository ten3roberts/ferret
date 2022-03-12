<script>
  import auth from "./service";
  import { isAuthenticated, user } from "../stores";

  import Profile from "./Profile.svelte";
</script>

<div>
  {#if $isAuthenticated}
    <Profile user={$user} />
  {:else}
    <button on:click={auth.login}>Login</button>
  {/if}
</div>
