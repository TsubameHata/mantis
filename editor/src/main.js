import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import App from "./App.vue";

const i18n = createI18n({
    locale: "cn",
    fallbackLocale: "en",
    messages: {
        en: {
            test: "Mantis Editor"
        },
        cn: {
            test: "Mantis 编辑器"
        }
    }
})

const app = createApp(App);
app.use(i18n)
app.mount("#app");