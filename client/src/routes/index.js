// /** @type {import('../lib/Posts.svelte').RequestHandler} */
export async function get() {
  return {
    status: 200,
    body: { posts: [{ title: "Foo", body: ["Bar Baz"] }] }
  }
}
