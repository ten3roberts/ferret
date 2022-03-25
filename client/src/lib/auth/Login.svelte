<script>
  import auth from "./service";
  import { isAuthenticated, user } from "../stores";

  import Profile from "./Profile.svelte";
  import Button from "../Button.svelte";
</script>

<div>
  {#if $isAuthenticated}
    <Profile user={$user} />
  {:else}
    <Button on:click={auth.login} text="Login" />
  {/if}
</div>
