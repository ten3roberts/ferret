import { backend_url } from "$lib/stores"

export async function post({ request }) {
  let { title, body, post_id, token } = await request.json();
  const response = await fetch(backend_url + "/create_comment", {
    method: "POST",
    body: JSON.stringify(
      {
        title,
        body,
        post_id
      }
    ),
    headers: { "content-type": "application/json", "Authorization": "Bearer " + token }
  })

  return {
    status: 303,
    body: await response.json()
  };
}

export async function del({ request }) {
  let { id, token } = await request.json()
  const response = await fetch(backend_url + "/comment/" + id, {
    method: "DELETE",
    headers: { "content-type": "application/json", "Authorization": "Bearer " + token }
  })

  return response
}
