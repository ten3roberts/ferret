<script>
  import Container from "$lib/Container.svelte";
  import { backend_url } from "../lib/stores";
</script>

<Container>
  <form
    class="p-5 bg-slate-100 flex flex-col w-2/3 m-auto h-2/3"
    action={"http://" + backend_url + "/create_post"}
    method="post"
  >
    <input
      class="font-bold bg-emerald-200 p-2 rounded-t-lg font-sans"
      type="text"
      name="title"
      id="title"
      placeholder="Title"
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
      />
    </div>
    <input type="submit" value="Submit" name="Submit" id="submit" />
  </form>
</Container>
