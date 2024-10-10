import { defineConfig } from "@rsbuild/core";
import { pluginVue } from "@rsbuild/plugin-vue";
import { pluginSass } from "@rsbuild/plugin-sass";
import path from "path";

export default defineConfig({
    html: {
        template: "./index.html",
    },
    source: {
        entry: {
            index: "./src/main.ts",
        },
        alias: {
            "@": path.resolve(__dirname, "src"),
        },
    },
    output: {
        cleanDistPath: true,
        copy: [
            {
                from: "node_modules/@isolovr/app-worker/app_worker.js",
                to: "./assets",
            },
            {
                from: "node_modules/@isolovr/app-worker/app_worker_bg.wasm",
                to: "./assets",
            },
            {
                from: "node_modules/@isolovr/ui-worker/ui_worker.js",
                to: "./assets",
            },
            {
                from: "node_modules/@isolovr/ui-worker/ui_worker_bg.wasm",
                to: "./assets",
            },
        ],
    },
    server: {
        port: 8301,
        host: "0.0.0.0",
    },

    tools: {
        rspack(config, { addRules }) {

            addRules([


            ]);

            config.resolveLoader = {
                alias: {
                    'worker-loader': require.resolve('worker-rspack-loader'),
                },
            };
            config.ignoreWarnings = [
                /ModuleWarning: Deprecation/,
                /ModuleWarning: \`alert-variant/,
                /ModuleWarning: \`avatar/,
            ];
            return config;
        },
    },
    plugins: [
        pluginVue(),
        pluginSass({
            sassLoaderOptions: {
                implementation: require.resolve("sass"),
                sourceMap: true,
            },
        }),
    ],
});
