const c = [
	() => import("../../src/routes/__layout.svelte"),
	() => import("../runtime/components/error.svelte"),
	() => import("../../src/routes/index.svelte"),
	() => import("../../src/routes/create_post.svelte"),
	() => import("../../src/routes/Post.svelte"),
	() => import("../../src/routes/post/[id].svelte")
];

const d = decodeURIComponent;

export const routes = [
	// src/routes/index.svelte
	[/^\/$/, [c[0], c[2]], [c[1]], null, ''],

	// src/routes/create_post.svelte
	[/^\/create_post\/?$/, [c[0], c[3]], [c[1]]],

	// src/routes/Post.svelte
	[/^\/Post\/?$/, [c[0], c[4]], [c[1]]],

	// src/routes/post/[id].svelte
	[/^\/post\/([^/]+?)\/?$/, [c[0], c[5]], [c[1]], (m) => ({ id: d(m[1])}), 'post/[id]']
];

// we import the root layout/error components eagerly, so that
// connectivity errors after initialisation don't nuke the app
export const fallback = [c[0](), c[1]()];