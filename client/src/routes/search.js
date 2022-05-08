import { backend_url } from "$lib/stores";

/** @type {import('./search').RequestHandler} */
export async function get({ request }) {
  let params = new URL(request.url).searchParams
  let text = params.get("text")
  console.log(`Url: ${params}  ${text}`)
  console.log("Searching for " + text)

  const response = await fetch(`${backend_url}/search?${params}`, { method: "GET" })
  return {
    status: response.status,
    body: await response.json()
  }
}
