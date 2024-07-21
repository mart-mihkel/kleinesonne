import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import adapter from "@sveltejs/adapter-node";

const IS_PROD = process.env.NODE_ENV === "production";

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),
    kit: {
        adapter: adapter(),
        csrf: {
            checkOrigin: IS_PROD,
        },
    },
};

export default config;
