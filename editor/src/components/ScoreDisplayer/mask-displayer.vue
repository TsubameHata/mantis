<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useDivLines, useMargin, useMaskBrush } from '../../store/appState';
import scoreStage from './score-stage.vue';

import colors from "../../colors";
const rgb = (c:number[])=>`rgb(${c[0]},${c[1]},${c[2]})`;

const {zIndex=52} = defineProps<{zIndex?:number}>();

const {divBlocks} = useDivLines();
const {left, right_w} = storeToRefs(useMargin());

const rectConfigs = computed(()=>divBlocks.map((el:any, index:number)=>{
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

const { brushActivated, nowEditing, brushRadius } = storeToRefs(useMaskBrush());
const brushColor = computed(()=>{
    return rgb(colors[nowEditing.value]);
});
const brushInStage = ref(false);
const showBrush = computed(()=>brushActivated.value&&brushInStage.value);

const pointerPos = reactive({x:0,y:0});

const mouseMove = (e:any)=>{
    const newPos = e.target.getStage().getPointerPosition()
    pointerPos.x = newPos.x
    pointerPos.y = newPos.y
};

// TEST
onMounted(()=>{brushActivated.value=true})

const brushConfig = computed(()=>{
    return {
        x: pointerPos.x,
        y: pointerPos.y,
        radius: brushRadius.value,
        stroke: 'red',
        fill: brushColor.value,
        strokeWidth: 2
    }
});
</script>

<template>
<score-stage :z-index="zIndex"
    @mousemove="mouseMove"
    @mouseenter="brushInStage = true"
    @mouseleave="brushInStage = false"
    >
    <v-layer>
        <v-rect v-for="config in rectConfigs" :config="config"></v-rect>
    </v-layer>
    <v-layer>
        <v-circle
            v-if="showBrush"
            :config="brushConfig"></v-circle>
    </v-layer>
</score-stage>
</template>

<style scoped>
</style>