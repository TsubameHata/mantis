<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useMaskBrush, useShouldUploadMask } from '../../store/appState';
import scoreStage from './score-stage.vue';

import colors from "../../colors";
import { VNodeRef } from 'vue';
import { useDiv, useMargin, useMasks, usePage } from '../../store/documentState';
import { upload_mask } from '../../requests';
const rgb = (c:number[])=>`rgb(${c[0]},${c[1]},${c[2]})`;

const {zIndex=52} = defineProps<{zIndex?:number}>();

const { openedPageDiv } = storeToRefs(useDiv());
const {left, right_w} = storeToRefs(useMargin()).openedPageMar.value.margin;

const rectConfigs = computed(()=>openedPageDiv.value.div.divBlocks.value.map((el:any, index:number)=>{
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
const pointerDown = ref(false);

const points = reactive<{value:number[]}>({value:[]});
const { openedPageMask } = storeToRefs(useMasks());

const stageRef = ref<VNodeRef | null>(null);

const mouseMove = (e:any)=>{
    const newPos = e.target.getStage().getPointerPosition();
    pointerPos.x = newPos.x;
    pointerPos.y = newPos.y;
    if(pointerDown.value){
        points.value.push(pointerPos.x);
        points.value.push(pointerPos.y);
        points.value = [...points.value];
    };
};

const mouseUp = ()=>{
    pointerDown.value = false;
    openedPageMask.value?.lines.push({
        index: nowEditing.value,
        points: [...points.value],
        width: brushRadius.value
    });
    points.value.splice(0, points.value.length);
};

const brushConfig = computed(()=>{
    return {
        x: pointerPos.x,
        y: pointerPos.y,
        radius: brushRadius.value,
        stroke: pointerDown.value?'red':'yellow',
        fill: brushColor.value,
        strokeWidth: 2
    };
});

const getOneLineConfig = (index:number, points:number[], radius:number)=>{
    return {
        points,
        stroke: rgb(colors[index]),
        lineCap: "round",
        lineJoin: "round",
        strokeWidth: radius*2
    };
};

const maskLinesConfig = computed(()=>{
    let config = openedPageMask.value?.lines.map(maskLine=>{
        return getOneLineConfig(
            maskLine.index,
            maskLine.points,
            maskLine.width
        );
    });
    return config;
});

const {umSignal} = storeToRefs(useShouldUploadMask());
watch(umSignal, async ()=>{
    const imgBlob = await stageRef.value.getImgBlob();
    upload_mask(imgBlob);
});
</script>

<template>
<score-stage :z-index="zIndex"
    ref="stageRef">
    <v-layer>
        <v-rect v-for="config in rectConfigs" :config="config"></v-rect>
    </v-layer>
    <v-layer>
        <v-line v-for="config in maskLinesConfig" :config="config"></v-line>
        <v-line :config="getOneLineConfig(nowEditing, points.value, brushRadius)"></v-line>
    </v-layer>
</score-stage>
<score-stage :z-index="zIndex+1"
    @mousemove="mouseMove"
    @mouseup="mouseUp"
    @mousedown="pointerDown = true"
    @mouseenter="brushInStage = true"
    @mouseleave="brushInStage = false">
    <v-layer>
        <v-circle
            v-if="showBrush"
            :config="brushConfig"></v-circle>
    </v-layer>
</score-stage>
</template>

<style scoped>
</style>