import { backend_url } from "$lib/stores";

/** @type {import('./[post_id]').RequestHandler} */
export async function get({ params }) {
  // let { id } = await request.request.json();
  // let params = await request.request;
  return await fetch(backend_url + "/post/" + params.post_id, { method: "GET" })
    .then(async v => {
      if (v.ok) {
        return { body: await v.json() }
      } else {
        return v
      }
    })
}

export async function del({ params }) {
  let { id } = params
  const response = await fetch(backend_url + "/post/" + id, {
    method: "DELETE",
  })

  if (response.ok) {
    return { status: 303, headers: { location: "/" } }
  } else {
    return { status: 404 }
  }
}
