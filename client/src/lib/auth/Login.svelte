<script>
  import auth from "./service";

  import Profile from "./Profile.svelte";
  import Button from "../Button.svelte";
  import { isAuthenticated, user } from "../stores";
</script>

<div>
  {#await auth.createClient()}
    <span class="p-2">Awaiting login</span>
  {:then { }}
    {#if $isAuthenticated}
      <div class="flex row">
        <Profile user={$user} />
        <Button on:click={auth.logout} text="Logout" />
      </div>
    {:else}
      <Button on:click={auth.login} text="Login" />
    {/if}
  {/await}
</div>
