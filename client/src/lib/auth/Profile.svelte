<script>
  export let user;
  import auth from "./service";
  /* export let displayName; */
  /* export let photoURL; */
  /* export let uid; */
</script>

<div class="flex justify-evenly gap-2 items-center">
  {#if user.picture}
    <img src={user.picture} class="w-8 rounded-full" alt="profilePicture" />
  {/if}
  <h3>{user.name}</h3>
  <button on:click={auth.logout}>Logout</button>
  <h4 class="divide-gray-100">{user.user_id}</h4>
</div>
