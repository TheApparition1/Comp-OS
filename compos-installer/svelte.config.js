import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			pages: 'dist',
			assets: 'dist',
			// Tauri loads a static build; use SPA fallback routing.
			fallback: 'index.html',
			precompress: false,
			strict: true
		})
	}
};

export default config;
