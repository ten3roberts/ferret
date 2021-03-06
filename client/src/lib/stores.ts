import { page } from "$app/stores";
import { writable, derived, type Writable } from "svelte/store"

export const posts = writable([{
  title: "How do I create a post?",
  content: ["Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis."]
}])

export const backend_url = "http://127.0.0.1:13000"

export const isAuthenticated = writable(false);
export const user: Writable<User> = writable({});
export const searchQuery: Writable<string> = writable()
export const popupOpen = writable(false);
export const error = writable();
