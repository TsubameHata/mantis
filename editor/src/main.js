import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import App from "./App.vue";

const i18n = createI18n({
    locale: "jp",
    fallbackLocale: "en",
    messages: {
        en: {
            test: "Mantis Editor",
            increment: "increment",
        },
        cn: {
            test: "Mantis 编辑器",
            increment: "增加",
        },
        jp: {
            test: "マンティス=エディタ",
            increment: "インクリメント",
        }
    }
})

const app = createApp(App);
app.use(i18n)
app.mount("#app");