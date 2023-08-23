import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter(),
		prerender: {
			// use relative URLs similar to an anchor tag <a href="/test/1"></a>
			// do not include group layout folders in the path such as /(group)/test/1
			entries: [
				'/',
				'/to_server',
				'/user_records/acc',
				'/user_records/rej',
				'/user_records/rem',
				'/server_records/acc',
				'/server_records/rej',
				'/server_records/rem'
			]
		}
	},
	preprocess: vitePreprocess()
};

export default config;
