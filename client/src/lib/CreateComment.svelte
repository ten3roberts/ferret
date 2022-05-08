<script>
  import auth from "$lib/auth/service";
  import Card from "$lib/Card.svelte";

  export let post_id;
  export let on_create = (_) => {};

  async function handleSubmit(title, body) {
    const submit = await fetch("/api/comments", {
      method: "POST",
      body: JSON.stringify({
        title,
        body,
        post_id,
        token: await auth.getToken(),
      }),
    }).then(async (v) => {
      if (v.redirected) {
        location.href = v.url;
      } else {
        let json = await v.json();
        on_create(json);
      }
    });
  }
</script>

<Card title={null} editable={handleSubmit} />
