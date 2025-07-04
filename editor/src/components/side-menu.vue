<script setup lang="ts">
import { MenuProps } from "ant-design-vue";
import { useI18n } from "vue-i18n";

import SolarCursorLinear from 'virtual:icons/solar/cursor-linear';
import SolarPenLinear from 'virtual:icons/solar/pen-linear';
import IconParkDividingLine from 'virtual:icons/icon-park/dividing-line';

const { t } = useI18n();

const items = ref<MenuProps["items"]>([
    {
        key: "cursor",
        label: t("tools.cursor"),
        icon: ()=>h(SolarCursorLinear)
    },
    {
        key: "div",
        label: t("tools.div"),
        icon: ()=>h(IconParkDividingLine)
    },
    {
        key: "mask",
        label: t("tools.mask"),
        icon: ()=>h(SolarPenLinear)
    }
]);

const selectedKeys = ref(["cursor"])
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