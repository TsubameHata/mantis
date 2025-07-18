<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useImgGeometry } from "../../store/documentState";

const emit = defineEmits<{
    click:[e:MouseEvent],
    mousemove:[e:MouseEvent],
    mousedown:[e:MouseEvent],
    mouseup:[e:MouseEvent],
    mouseenter:[e:MouseEvent],
    mouseleave:[e:MouseEvent]
}>();

const imgGeometry = useImgGeometry();

const {width,height} = storeToRefs(imgGeometry);

const { zIndex=10 } = defineProps<{
    zIndex?: number
}>();
const stageStyle = 
    computed(()=>`position:absolute;top:0;left:0;opacity:50%;z-index:${zIndex}`);

const stageRef = ref()

const getImgURI = ()=>stageRef.value.getNode().toDataURL();
const getImgBlob = ()=>stageRef.value.getNode().toBlob();
defineExpose({getImgURI, getImgBlob});

</script>

<template>
<v-stage
    :config="{width, height}"
    :style="stageStyle"
    @click="(e: MouseEvent) => { emit('click', e) }"
    @mousemove="(e: MouseEvent) => { emit('mousemove', e) }"
    @mousedown="(e: MouseEvent) => {emit('mousedown', e)}"
    @mouseup="(e: MouseEvent) => {emit('mouseup', e)}"
    @mouseenter="(e: MouseEvent) => {emit('mouseenter', e)}"
    @mouseleave="(e: MouseEvent) => {emit('mouseleave', e)}"
    ref="stageRef"
    id="score-stage">
    <slot></slot>
</v-stage>
</template>

<style scoped>
</style>