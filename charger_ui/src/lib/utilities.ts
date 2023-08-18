import { writable, type Writable } from "svelte/store";

const uid_store: Writable<number> = writable(0);

/**
 * Increments an internal counter to generate unique identifiers and guarantees uniqueness.
 * @returns unique identifier string.
 */
export function make_uid(): string {
    let id = 0;
    uid_store.update((cur) => {
        cur++;
        id = cur;
        return cur;
    });
    return `id_${id}`;
}
