<script>
  import Container from "$lib/Container.svelte";
  import { backend_url } from "../lib/stores";
  import auth from "$lib/auth/service";

  let title = "";
  let body = "";
  async function handleSubmit() {
    const submit = await fetch("/api/posts", {
      method: "POST",
      body: JSON.stringify({
        title: title,
        body: body,
      }),
      /* headers: { "content-type": "application/json" }, */
    });
  }
</script>

<Container>
  <form
    on:submit|preventDefault={handleSubmit}
    class="p-5 bg-slate-100 flex flex-col w-2/3 m-auto h-2/3"
  >
    <input
      class="font-bold bg-emerald-200 p-2 rounded-t-lg font-sans"
      type="text"
      name="title"
      placeholder="Title"
      bind:value={title}
    />
    <div
      class="bg-gray-100 p-2 font-serif leading-relaxed rounded-b-lg shadow
      h-full"
    >
      <textarea
        class="w-full h-full p-2 font-serif, leading-relaxed"
        type="text"
        name="body"
        id="body"
        bind:value={body}
      />
    </div>
    <input type="submit" value="Submit" name="Submit" id="submit" />
  </form>
</Container>
