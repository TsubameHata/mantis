<script setup lang="ts">
import { MenuProps } from "ant-design-vue"
import { MenuClickEventHandler, MenuInfo } from "ant-design-vue/es/menu/src/interface";
import { useI18n } from "vue-i18n"
import { new_session, get_sessions } from "../requests";

const { t } = useI18n();

const items = ref<MenuProps["items"]>([
    // {
    //     key: "files",
    //     label: t("menu.files.files"),
    //     children: [
    //         {
    //             key: "new",
    //             label: t("menu.files.new")
    //         },
    //         {
    //             key: "open",
    //             label: t("menu.files.open")
    //         }
    //     ]
    // },
    {
        key: "help",
        label: t("menu.help.help"),
        children: [
            {
                key: "doc",
                label: t("menu.help.doc")
            },
            {
                key: "bug",
                label: t("menu.help.bug")
            },
            {
                key: "github",
                label: t("menu.help.github")
            }
        ]
    }
]);

const onClick: MenuClickEventHandler = (e:MenuInfo)=>{
    if(e.key=="new") new_session();
    else if (e.key=="open") get_sessions();
}
</script>

<template>
    <div class="logo">MANTIS</div>
    <a-divider 
        type="vertical" 
        style="height: 80%; align-self: center;"></a-divider>
    <a-menu
        mode="horizontal"
        :selectable="false"
        triggerSubMenuAction="hover"
        :items="items"
        @click="onClick"></a-menu>
</template>

<style scoped>
.logo {
    padding-left: 20px;
    padding-right: 20px;
    font-size: x-large;
    text-align: center;
    vertical-align: center;
    font-family: Georgia, 'Times New Roman', Times, serif;
    user-select: none;
    cursor: pointer;
}
</style>