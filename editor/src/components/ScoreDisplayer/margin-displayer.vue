<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useMargin_ } from '../../store/appState';
import { useImgGeometry, useMargin, usePage } from '../../store/documentState';

import ScoreLine from "./score-line.vue"

const {zIndex=50} = defineProps<{zIndex?:number}>();

const {left, right_w, top, bottom_h} = toRefs(storeToRefs(useMargin()).openedPageMar.value.margin);
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