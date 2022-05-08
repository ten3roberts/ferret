const c = [
	() => import("../../src/routes/__layout.svelte"),
	() => import("../runtime/components/error.svelte"),
	() => import("../../src/routes/index.svelte"),
	() => import("../../src/routes/create_post.svelte"),
	() => import("../../src/routes/search.svelte"),
	() => import("../../src/routes/about.svelte"),
	() => import("../../src/routes/post/[post_id].svelte"),
	() => import("../../src/routes/user/[user_id].svelte")
];

const d = decodeURIComponent;

export const routes = [
	// src/routes/index.svelte
	[/^\/$/, [c[0], c[2]], [c[1]]],

	// src/routes/create_post.svelte
	[/^\/create_post\/?$/, [c[0], c[3]], [c[1]]],

	// src/routes/search.svelte
	[/^\/search\/?$/, [c[0], c[4]], [c[1]], null, 'search'],

	// src/routes/about.svelte
	[/^\/about\/?$/, [c[0], c[5]], [c[1]]],

	// src/routes/post/[post_id].svelte
	[/^\/post\/([^/]+?)\/?$/, [c[0], c[6]], [c[1]], (m) => ({ post_id: d(m[1])}), 'post/[post_id]'],

	// src/routes/user/[user_id].svelte
	[/^\/user\/([^/]+?)\/?$/, [c[0], c[7]], [c[1]], (m) => ({ user_id: d(m[1])}), 'user/[user_id]']
];

// we import the root layout/error components eagerly, so that
// connectivity errors after initialisation don't nuke the app
export const fallback = [c[0](), c[1]()];