import { error } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export function load({ params }) {
	if (params.slug === 'acc') {
		return {
			photo_status: 'REV_ACC'
		};
	} else if (params.slug === 'rej') {
		return {
			photo_status: 'REV_REJECT'
		};
	} else if (params.slug === 'rem') {
		return {
			photo_status: 'NOT_REV'
		};
	}

	throw error(404, 'Not found');
}
/*export function entries() {
	return [{ slug: 'acc' }, { slug: 'rej' }, { slug: 'rem' }];
}*/

export const prerender = true;
