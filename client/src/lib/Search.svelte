<script>
  import Button from "./Button.svelte";

  export let text = "";
  function search() {
    console.log("Searching");
    const params = new URLSearchParams({
      text,
    });

    const url = `/search?${params}`;
    console.log(url);
    location.href = url;
  }
</script>

<form on:submit|preventDefault={(_) => search()}>
  <input
    class="bg-dark text-light rounded-md p-2"
    placeholder="Search"
    bind:value={text}
  />
  <Button text="Search" type="submit" />
</form>
