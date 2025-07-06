<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useMargin } from '../../store/appState';
import { useImgGeometry } from '../../store/documentState';

import ScoreLine from "./score-line.vue"

const margin = useMargin();
const {left, right, top, bottom} = storeToRefs(margin);

const imgGeometry = useImgGeometry();
const {width,height} = storeToRefs(imgGeometry);

const bottom_h = computed({
    get: ()=>height.value-bottom.value,
    set: (n:number)=>{
        bottom.value = height.value - n
    }
});

const right_w = computed({
    get: ()=>width.value-right.value,
    set: (n:number)=>{
        right.value = width.value - n
    }
});

const marginDisplayerConfig = {
    stroke: "rgba(0,0,0,1)",
    strokeWidth: 3,
    points: [0,0],
    dash: [20,10,3,10],
    draggable: true
};

const getVerticalLineConfigCurried = (v: Ref<number>)=>()=>{
    let config = {...marginDisplayerConfig};
    config["stroke"] = "rgb(0,255,0)";
    config["points"] = [v.value, 0, v.value, height.value];
    return config;
};

const leftLineConfig = computed(getVerticalLineConfigCurried(left));
const rightLineConfig = computed(getVerticalLineConfigCurried(right_w));
</script>

<template>
<score-stage>
    <v-layer>
        <score-line v-model:height="top"></score-line>
        <score-line v-model:height="bottom_h"></score-line>
        <score-line :vertical="true" v-model:height="left"></score-line>
        <score-line :vertical="true" v-model:height="right_w"></score-line>
    </v-layer>
</score-stage>
</template>

<style scoped>
</style>