import { backend_url } from "$lib/stores";

/** @type {import('./[id]').RequestHandler} */
export async function get({ params }) {
  // let { id } = await request.request.json();
  // let params = await request.request;
  return await fetch(backend_url + "/post/" + params.post_id, { method: "GET" })
    .then(async v => {
      if (v.ok) {
        return v.json()
      } else {
        return Promise.reject(await v.text())
      }
    })
    .then(v => { return { body: v } }, _ => { return { status: 404 } })
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
