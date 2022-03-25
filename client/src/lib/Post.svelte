<script>
  export let title = "";
  export let body = [];
</script>

<div class="p-5 w-2/3 text-lg">
  <h2 class="font-bold font-sans bg-emerald-200 p-2 rounded-t-lg">
    Q: {title}
  </h2>
  <div class="bg-gray-100 p-2 font-serif leading-relaxed rounded-b-lg shadow">
    {#each body as p}
      <p>{p}</p>
    {/each}
  </div>
</div>
