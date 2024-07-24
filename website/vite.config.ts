import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

export default defineConfig({
    plugins: [sveltekit()],
    server: {
        proxy: {
            "/api": {
                target: "http://api:3000",
                rewrite: (path) => path.replace(/^\/api/, ""),
            },
        },
    },
});
