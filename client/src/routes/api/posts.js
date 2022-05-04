import { backend_url } from "$lib/stores"
/** @type {import('./posts').RequestHandler} */
export async function post({ request }) {
  let { title, body, token } = await request.json();
  const response = await fetch(backend_url + "/create_post", {
    method: "POST",
    body: JSON.stringify(
      {
        title,
        body,
      }
    ),
    headers: { "content-type": "application/json", "Authorization": "Bearer " + token }
  })
    .then(async v => { if (v.ok) return v.json(); else return Promise.reject(await v.text()) })
    .then(v => JSON.parse(v))

  return {
    status: 303,
    headers: {
      location: `/post/${response.post.post_id}`
    }
  };
}

/** @type {import('./posts').RequestHandler} */
export async function get() {
  console.log("Getting posts")
  return await fetch(backend_url + "/posts", {
    method: "GET",
  })
}

export async function del({ request }) {
  let { id, token } = await request.json()
  console.log("deleting post " + id)
  const response = await fetch(backend_url + "/post/" + id, {
    method: "DELETE",
    headers: { "content-type": "application/json", "Authorization": "Bearer " + token }
  })

  return response
}
