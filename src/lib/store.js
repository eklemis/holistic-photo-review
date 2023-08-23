//@ts-nocheck
import { writable } from 'svelte/store';

export const toServer = writable({
	server: '',
	port: 0,
	user_id: 0
});
export const photoPath = writable('F:\\Indonesia');
export const masterRetrieved = writable(false);
