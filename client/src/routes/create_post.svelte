<script>
  import Container from "$lib/Container.svelte";
  import { backend_url } from "../lib/stores";
  import auth from "$lib/auth/service";
  import Card from "$lib/Card.svelte";

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
  <Card editable={handleSubmit} />
</Container>
