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