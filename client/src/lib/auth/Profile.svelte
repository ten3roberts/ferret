<script>
  export let user;
  import auth from "./service";
</script>

<div class="flex justify-start gap-2 p-2 items-center m-2">
  {#if user.picture}
    <img
      src={user.picture}
      class="w-8 rounded-full bg-purple-500"
      alt="profilePicture"
    />
  {/if}
  <a href={"/user/" + user.user_id} class="text-light">{user.name}</a>
</div>
