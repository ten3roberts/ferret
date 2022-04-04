<script>
  export let text;
  export let type = "button";
</script>

<button
  class="bg-dark text-light py-2 px-3 hover:bg-green-500 hover:text-dark m-1 rounded-full"
  {type}
  on:click>{text}</button
>
