import { backend_url } from "$lib/stores";

/** @type {import('./search').RequestHandler} */
export async function get({ request }) {
  let params = new URL(request.url).searchParams
  let searchQuery = params.get("searchQuery")
  let page = params.get("page");
  console.log(`Url: ${params}  ${searchQuery}`)
  console.log("Searching for " + searchQuery + ":" + page)

  const response = await fetch(`${backend_url}/search?${params}`, { method: "GET" })
  return {
    status: response.status,
    body: await response.json()
  }
}
