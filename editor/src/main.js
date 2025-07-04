import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import App from "./App.vue";

import en_us from "./locales/en_us.json";
import zh_cn from "./locales/zh_cn.json";

const i18n = createI18n({
    locale: "zh_cn",
    fallbackLocale: "en_us",
    messages: {
        en_us,
        zh_cn
    }
})

const app = createApp(App);
app.use(i18n)
app.mount("#app");