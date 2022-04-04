import { backend_url } from "$lib/stores";

/** @type {import('./[id]').RequestHandler} */
export async function get({ params }) {
  // let { id } = await request.request.json();
  // let params = await request.request;
  return await fetch(backend_url + "/post/" + params.id, { method: "GET" })
    .catch(v => { console.log("Failed: ", v); return v })
    .then(v => v.json())
    .then(v => { return { body: v } })
  // const { id } = params;
  // const res = await fetch(backend_url + "/post/" + id, {
  //   method: "GET",
  //   accept: "application/json"
  // }).then(v => v.json());

  // console.log("Got: " + res);

  // return { body: { title: "Foo", body:" Bar" }}
}
