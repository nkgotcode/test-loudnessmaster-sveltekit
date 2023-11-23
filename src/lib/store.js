import { writable } from 'svelte/store';

export const loaded = writable(false);
export const process = writable(false);
export const results = writable([]);
export const draggedResults = writable(null);
export const processingType = writable('Offline');
