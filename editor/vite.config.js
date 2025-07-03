import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite"

export default defineConfig({
    plugins: [
        vue(),
        Components({}),
        AutoImport({
            imports: [
                "vue",
                "vue-i18n"
            ],
            dts: true
        })
    ],
    server: {
        proxy: {
            "api/": {
                target: "http://127.0.0.1:14513",
                changeOrigin: true,
                rewrite: (path)=> path.replace(/^\/api/, ""),
                ws: true
            }
        }
    }
});
