<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useDivLines, useMargin } from '../../store/appState';
import scoreStage from './score-stage.vue';

import colors from "../../colors";
const rgb = (c:number[])=>`rgb(${c[0]},${c[1]},${c[2]})`;

const {zIndex=52} = defineProps<{zIndex?:number}>();

const {divLines} = useDivLines();
const {top, bottom_h, left, right_w} = storeToRefs(useMargin());
const divBlocks = computed(()=>{
    if(divLines[0]==undefined) return [top.value, bottom_h.value];
    let blocks: number[][] = [];
    blocks.push([top.value, divLines[0]]);
    for(let i=0; i<divLines.length-1; i++){
        blocks.push([divLines[i], divLines[i+1]]);
    };
    blocks.push([divLines[divLines.length-1], bottom_h.value]);
    return blocks;
});
const rectConfigs = computed(()=>divBlocks.value.map((el:any, index:number)=>{
    const h = el[1]-el[0];
    const w = right_w.value - left.value;
    return {
        x: left.value,
        y: el[0],
        width: w,
        height: h,
        fill: rgb(colors[index]) 
    };
}));
</script>

<template>
<score-stage :z-index="zIndex">
    <v-layer>

    </v-layer>
    <v-layer>
        <v-rect v-for="config in rectConfigs" :config="config"></v-rect>
    </v-layer>
</score-stage>
</template>

<style scoped>
</style>