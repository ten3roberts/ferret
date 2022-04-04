import { backend_url } from "$lib/stores"
/** @type {import('./posts').RequestHandler} */
export async function post(request) {
  let { title, body } = await request.request.json();
  return await fetch(backend_url + "/create_post", {
    method: "POST",
    body: JSON.stringify(
      {
        title,
        body,
      }
    ),
    headers: { "content-type": "application/json" }
  })
}

/** @type {import('./posts').RequestHandler} */
export async function get() {
  console.log("Getting posts")
  return await fetch(backend_url + "/posts", {
    method: "GET",
  })
}
