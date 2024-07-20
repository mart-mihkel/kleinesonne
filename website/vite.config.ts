import { enhancedImages } from "@sveltejs/enhanced-img";
import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

export default defineConfig({
    plugins: [enhancedImages(), sveltekit()],
    server: {
        proxy: {
            "/api": {
                target: "http://api:3000",
                rewrite: (path) => path.replace(/^\/api/, ""),
            },
        },
    },
});
