<script setup lang="ts">
import { MenuProps } from "ant-design-vue";
import { useI18n } from "vue-i18n";

import SolarCursorLinear from 'virtual:icons/solar/cursor-linear';

const { t } = useI18n();

const items = ref<MenuProps["items"]>([
    {
        key: "cursor",
        label: t("tools.cursor"),
        icon: ()=>h(SolarCursorLinear)
    }
]);
</script>

<template>
<a-menu
    mode="inline"
    :items="items"
    :multiple="false"></a-menu>
</template>

<style scoped>
</style>