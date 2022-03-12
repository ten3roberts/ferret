// /** @type {import('../lib/Posts.svelte').RequestHandler} */
export async function get() {
  return {
    status: 200,
    body: { posts: [{ title: "Foo", content: ["Bar Baz"] }] }
  }
}
