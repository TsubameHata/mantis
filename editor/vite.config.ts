import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite"
import { AntDesignVueResolver } from "unplugin-vue-components/resolvers"
import Icons from "unplugin-icons/vite"

export default defineConfig({
    plugins: [
        vue(),
        Components({
            dirs: ["src/components"],
            resolvers: [
                AntDesignVueResolver({
                    importStyle: false
                })
            ],
            dts: true
        }),
        AutoImport({
            imports: [
                "vue",
                "vue-i18n"
            ],
            dts: true
        }),
        Icons({
            compiler: "vue3",
        })
    ],
    server: {
        proxy: {
            "/api": {
                target: "http://127.0.0.1:11451",
                changeOrigin: true,
                // rewrite: (path)=> path.replace(/^\/api/, ""),
                ws: true
            }
        }
    }
});
