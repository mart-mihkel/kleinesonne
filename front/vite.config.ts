import { sveltekit } from '@sveltejs/kit/vite';
import { enhancedImages } from '@sveltejs/enhanced-img';
import { imagetools } from '@zerodevx/svelte-img/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [
		enhancedImages(),
		imagetools(),
		sveltekit()
	],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
