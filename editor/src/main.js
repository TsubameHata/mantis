import { createApp } from "vue";
import { createPinia } from "pinia";
import { createI18n } from "vue-i18n";
import VueKonva from "vue-konva";
import App from "./App.vue";
import "./main.css"

import en_us from "./locales/en_us.json";
import zh_cn from "./locales/zh_cn.json";

const app = createApp(App);

app.use(VueKonva)

const i18n = createI18n({
    locale: "en_us",
    fallbackLocale: "en_us",
    legacy: false,
    messages: {
        en_us,
        zh_cn
    }
});
app.use(i18n)

const pinia = createPinia()
app.use(pinia)

app.mount("#app");