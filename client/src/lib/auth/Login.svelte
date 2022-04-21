<script>
  import auth from "./service";
  import { isAuthenticated, user } from "../stores";

  import Profile from "./Profile.svelte";
  import Button from "../Button.svelte";
</script>

<div>
  {#if $isAuthenticated}
    <div class="flex row">
      <Profile user={$user} />
      <Button on:click={auth.logout} text="Logout" />
    </div>
  {:else}
    <Button on:click={auth.login} text="Login" />
  {/if}
</div>
