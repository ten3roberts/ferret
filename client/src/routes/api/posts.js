import { backend_url } from "$lib/stores"
/** @type {import('./posts').RequestHandler} */
export async function post(request) {
  let { title, body } = await request.request.json();
  const response = await fetch(backend_url + "/create_post", {
    method: "POST",
    body: JSON.stringify(
      {
        title,
        body,
      }
    ),
    headers: { "content-type": "application/json" }
  }).then(v => { console.log("Got:", v); return v.json() }).then(v => JSON.parse(v))

  console.log("Got: ", response, typeof response);

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
