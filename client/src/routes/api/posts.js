import { backend_url } from "$lib/stores"
/** @type {import('./posts').RequestHandler} */
export async function post({ request }) {
  let { title, body, token } = await request.json();
  let headers = request.headers;
  console.log("headers: ", headers)
  console.log("Token:" + token)
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
    .then(async v => { if (v.ok) return v; else return Promise.reject(await v.text()) })
    .then(v => v.json())
    .then(v => { console.log("Got: ", v); return v })
    .then(v => JSON.parse(v))

  return {
    status: 303,
    headers: {
      location: `/post/${response.id}`
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
