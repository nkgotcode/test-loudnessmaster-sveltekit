import { sentrySvelteKit } from '@sentry/sveltekit';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import vercel from 'vite-plugin-vercel';

export default defineConfig({
	plugins: [
		sentrySvelteKit({
			sourceMapsUploadOptions: {
				org: 'itsnk',
				project: 'javascript-sveltekit'
			}
		}),
		sveltekit(),
		vercel()
	],
	vercel: {}
});
