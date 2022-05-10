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
export async function get({ request }) {
  console.log("Getting posts")
  let params = new URL(request.url).searchParams
  return await fetch(`${backend_url}/posts?${params}`, {
    method: "GET",
  })
}

/** @type {import('./posts').RequestHandler} */
export async function patch({ request }) {
  let { post_id, comment_id, token } = await request.json();
  return fetch(backend_url + "/post/mark_solved", {
    method: "PATCH",
    headers: { "content-type": "application/json", "Authorization": "Bearer " + token },
    body: JSON.stringify({ post_id, comment_id })
  })

}

export async function del({ request }) {
  let { id, token } = await request.json()
  console.log("deleting post " + id)
  const response = await fetch(backend_url + "/post/" + id, {
    method: "DELETE",
    headers: { "content-type": "application/json", "Authorization": "Bearer " + token }
  })

  console.log("Got: " + response.status + ";" + response.headers)
  return { status: response.status, headers: response.headers }
}
