<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useMargin } from '../../store/documentState';

import ScoreLine from "./score-line.vue"

const {zIndex=50} = defineProps<{zIndex?:number}>();

const { openedPageMar:opm } = storeToRefs(useMargin());
</script>

<template>
<score-stage :z-index="zIndex">
    <v-layer>
        <score-line v-model:height="opm.margin.top.value"></score-line>
        <score-line v-model:height="opm.margin.bottom_h.value"></score-line>
        <score-line :vertical="true" v-model:height="opm.margin.left.value"></score-line>
        <score-line :vertical="true" v-model:height="opm.margin.right_w.value"></score-line>
    </v-layer>
</score-stage>
</template>

<style scoped>
</style>