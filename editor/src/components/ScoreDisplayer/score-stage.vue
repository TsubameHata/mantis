<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useImgGeometry } from "../../store/documentState";

const emit = defineEmits<{click:[e:MouseEvent]}>();

const imgGeometry = useImgGeometry();

const {width,height} = storeToRefs(imgGeometry);

const { zIndex=10 } = defineProps<{
    zIndex?: number
}>();
const stageStyle = 
    computed(()=>`position:absolute;top:0;left:0;opacity:50%;z-index:${zIndex}`);
</script>

<template>
<v-stage
    :config="{width, height}"
    :style="stageStyle"
    @click="(e: MouseEvent) => { emit('click', e) }"
    >
    <slot></slot>
</v-stage>
</template>

<style scoped>
</style>