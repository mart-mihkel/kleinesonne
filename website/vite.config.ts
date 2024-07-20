import { sveltekit } from "@sveltejs/kit/vite";
import { enhancedImages } from "@sveltejs/enhanced-img";
import { defineConfig } from "vite";

const DEV_API = "http://localhost:3000";

export default defineConfig({
    plugins: [enhancedImages(), sveltekit()],
    server: {
        proxy: {
            "/api": {
                target: DEV_API,
                rewrite: (path) => path.replace(/^\/api/, ""),
            },
        },
    },
});
