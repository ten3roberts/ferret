<script>
  export let user;
  import auth from "./service";
  /* export let displayName; */
  /* export let photoURL; */
  /* export let uid; */
  /* <button on:click={auth.logout}>Logout</button> */
</script>

<div class="flex justify-evenly gap-2 items-center m-2">
  {#if user.picture}
    <img
      src={user.picture}
      class="w-8 rounded-full bg-purple-500"
      alt="profilePicture"
    />
  {/if}
  <h3 class="text-light">{user.name}</h3>
</div>
