import { backend_url } from "$lib/stores";

/** @type {import('./[user_id]').RequestHandler} */
export async function get({ params }) {
  return await fetch(backend_url + "/user/" + params.user_id, { method: "GET" })
    .then(async v => {
      if (v.ok) { return { body: await v.json() } }
      else { v }
    })
}
