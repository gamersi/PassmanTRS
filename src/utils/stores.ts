import { writable } from "svelte/store";

export const masterPassword = writable("");
export const passwords = writable([]);