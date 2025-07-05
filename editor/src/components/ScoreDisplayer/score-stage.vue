<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useImgGeometry } from "../../store/pageState";

const imgGeometry = useImgGeometry();

const {width,height} = storeToRefs(imgGeometry);

const { zIndex } = defineProps<{
    zIndex?: number
}>();

const z = computed(()=>zIndex?zIndex:10);
const stageStyle = computed(()=>`position:absolute;top:0;left:0;opacity:50%;z-index:${z}`);
</script>

<template>
<v-stage
    :config="{width, height}"
    :style="stageStyle"
    >
    <slot></slot>
</v-stage>
</template>

<style scoped>
</style>