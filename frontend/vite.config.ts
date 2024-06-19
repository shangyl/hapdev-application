import { defineConfig } from "vite";
import { fileURLToPath } from "url";
import vue from "@vitejs/plugin-vue";
import ViteRsw from "vite-plugin-rsw";
import { viteStaticCopy } from "vite-plugin-static-copy";

import { resolve } from "path";

// https://vitejs.dev/config/
export default defineConfig({
    resolve: {
        alias: {
            "@": fileURLToPath(new URL("./src", import.meta.url)),
        },
    },
    css: {
        preprocessorOptions: {
            scss: {
                charset: false,
            },
        },
    },
    build: {
        rollupOptions: {
            //   external: ["web-user-wasm", "worker"],
            input: {
                main: resolve(__dirname, "index.html"),
            },
            output: {
                manualChunks: (id) => {
                    console.log("id", id);
                    const modules = ["frontend-wasm"];
                    const chunk = modules.find((module) => id.includes(`${module}/pkg`));
                    return chunk ? `${chunk}` : "";
                },
                entryFileNames: "assets/[name].js",
                chunkFileNames: "assets/[name].js",
                assetFileNames: "assets/[name].[ext]",
            },
            makeAbsoluteExternalsRelative: true,
        },
    },

    server: {
        port: 8000,
        host: "0.0.0.0",
    },
    plugins: [
        vue(),
        ViteRsw(),
        viteStaticCopy({
            targets: [
                {
                    src: "node_modules/@isolovr/app-worker/app_worker.js",
                    dest: "./assets",
                },
                {
                    src: "node_modules/@isolovr/app-worker/app_worker_bg.wasm",
                    dest: "./assets",
                },
                {
                    src: "node_modules/@isolovr/ui-worker/ui_worker.js",
                    dest: "./assets",
                },
                {
                    src: "node_modules/@isolovr/ui-worker/ui_worker_bg.wasm",
                    dest: "./assets",
                },
            ],
        }),
    ],
});
