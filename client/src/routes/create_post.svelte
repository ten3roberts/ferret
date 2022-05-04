<script>
  import Container from "$lib/Container.svelte";
  import auth from "$lib/auth/service";
  import { isAuthenticated } from "$lib/stores";
  import Card from "$lib/Card.svelte";

  async function handleSubmit(title, body) {
    const submit = await fetch("/api/posts", {
      method: "POST",
      body: JSON.stringify({
        title,
        body,
        token: await auth.getToken(),
      }),
    }).then((v) => {
      if (v.redirected) {
        location.href = v.url;
      }
    });
  }
</script>

<Container>
  {#if $isAuthenticated}
    <Card editable={handleSubmit} title="" />
  {/if}
</Container>
