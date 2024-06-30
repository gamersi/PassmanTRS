import { writable, type Writable } from "svelte/store";
import type { Password } from "./types";

export const masterPassword = writable("");
export const passwords: Writable<Password[]> = writable([]);
export const theme = writable("light");
export const isSettingsOpen = writable(false);
export const search = writable("");
