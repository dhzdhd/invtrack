import gleam from "vite-gleam";

import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
// export default {
//     plugins: [gleam()],
//     build: {
//         rollupOptions: {
//             input: 'main.js',
//             output: {
//                 dir: './dist',
//                 entryFileNames: 'assets/converted.js',
//             }
//         }
//     },
//     // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
//     clearScreen: false,
//     // Tauri expects a fixed port, fail if that port is not available
//     server: {
//         port: 1420,
//         strictPort: true,
//         host: host || false,
//         hmr: host
//             ? {
//                 protocol: "ws",
//                 host,
//                 port: 1421,
//             }
//             : undefined,
//         watch: {
//             // 3. tell vite to ignore watching `src-tauri`
//             ignored: ["**/src-tauri/**"],
//         },
//     },
// };


export default defineConfig({
    plugins: [gleam()],
    build: {
        rollupOptions: {
            input: 'main.js',
            output: {
                dir: './dist',
                entryFileNames: 'assets/converted.js',
            }
        }
    },
    clearScreen: false,
    server: {
        port: 5173,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: 'ws',
                host,
                port: 1421,
            }
            : undefined,

        watch: {
            ignored: ['**/src-tauri/**'],
        },
    },
    // Env variables starting with the item of `envPrefix` will be exposed in tauri's source code through `import.meta.env`.
    envPrefix: ['VITE_', 'TAURI_ENV_*'],
    build: {
        target:
            process.env.TAURI_ENV_PLATFORM == 'windows'
                ? 'chrome105'
                : 'safari13',
        minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,
        sourcemap: !!process.env.TAURI_ENV_DEBUG,
    },
});