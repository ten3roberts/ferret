<script>
  import auth from "$lib/auth/service";
  import Card from "$lib/Card.svelte";

  export let post_id;

  async function handleSubmit(title, body) {
    const submit = await fetch("/api/comments", {
      method: "POST",
      body: JSON.stringify({
        title,
        body,
        post_id,
        token: await auth.getToken(),
      }),
    }).then((v) => {
      if (v.redirected) {
        location.href = v.url;
      }
    });
  }
</script>

<div class="m-5">
  <Card title={null} editable={handleSubmit} />
</div>
