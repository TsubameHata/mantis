<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useMargin } from '../../store/appState';
import { useImgGeometry } from '../../store/documentState';

import ScoreLine from "./score-line.vue"

const {zIndex=50} = defineProps<{zIndex?:number}>();

const margin = useMargin();
const {left, right_w, top, bottom_h} = storeToRefs(margin);

const imgGeometry = useImgGeometry();
const {width,height} = storeToRefs(imgGeometry);
</script>

<template>
<score-stage :z-index="zIndex">
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