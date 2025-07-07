<script setup lang="ts">
import { storeToRefs } from 'pinia';
import SolarPenLinear from 'virtual:icons/solar/pen-linear';
import { useMaskBrush } from '../../store/appState';

const {index} = defineProps<{
    index: number
}>();

const { brushActivated, nowEditing } = storeToRefs(useMaskBrush());

const activated = computed(()=>{
    if(brushActivated.value && nowEditing.value==index) return true;
    else return false;
});

const containerClassNames = computed(()=>{
    if(activated.value){
        return "activated svg_container";
    } else {
        return "svg_container"
    }
});

const onClick = ()=>{
    if(activated.value){
        brushActivated.value = false;
    } else {
        brushActivated.value = true;
        nowEditing.value = index;
    }
}
</script>

<template>
<div :class="containerClassNames"
    @click="onClick">
    <SolarPenLinear></SolarPenLinear>
</div>
</template>

<style scoped>
    .activated {
        color: #1677ff;
    }
</style>