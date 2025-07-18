<script setup lang="ts">
import { MenuProps } from "ant-design-vue";
import { useI18n } from "vue-i18n";

import SolarCursorLinear from 'virtual:icons/solar/cursor-linear';
import SolarPenLinear from 'virtual:icons/solar/pen-linear';
import IconParkDividingLine from 'virtual:icons/icon-park/dividing-line';
import ProiconsPageMargins from 'virtual:icons/proicons/page-margins';
import PixelFaceThinking from 'virtual:icons/pixel/face-thinking';

import { useActivatedTool } from "../store/appState";

const { t } = useI18n();

const items = ref<MenuProps["items"]>([
    {
        key: "cursor",
        label: t("tools.cursor"),
        icon: ()=>h(SolarCursorLinear)
    },
    {
        key: "margin",
        label: t("tools.margin"),
        icon: ()=>h(ProiconsPageMargins)
    },
    {
        key: "div",
        label: t("tools.div"),
        icon: ()=>h(IconParkDividingLine)
    },
    {
        key: "detect",
        label: t("tools.detect"),
        icon: ()=>h(PixelFaceThinking)
    },
    {
        key: "mask",
        label: t("tools.mask"),
        icon: ()=>h(SolarPenLinear)
    }
]);

const activatedTool = useActivatedTool();

const selectedKeys = computed({
    get: () => [unref(activatedTool.activatedTool)],
    set: val => { activatedTool.activatedTool = val[0]; }
});
</script>

<template>
<a-menu
    mode="inline"
    :items="items"
    :multiple="false"
    v-model:selected-keys="selectedKeys"></a-menu>
</template>

<style scoped>
</style>