<script setup lang="ts">
import { storeToRefs } from 'pinia';
import ScoreDisplayer from './ScoreDisplayer/score-displayer.vue';
import splitDisplayer from './SplitDisplayer/split-displayer.vue';
import { useImgSrc } from '../store/documentState';
import { useActivatedTool } from '../store/appState';

const { pageURLPrefix } = storeToRefs(useImgSrc());

const { activatedTool } = storeToRefs(useActivatedTool());

const showScore = computed(()=>pageURLPrefix.value!=""&&activatedTool.value!="split");
const showSplitResult = computed(()=>activatedTool.value=="split");
</script>

<template>
    <div id="score_container">
        <score-displayer v-if="showScore"></score-displayer>
        <split-displayer v-else-if="showSplitResult"></split-displayer>
        <div class="empty_container" v-else><a-empty description=""></a-empty></div>
    </div>
</template>

<style scoped>
#score_container {
    display: flex;
    justify-content: center;
    align-items: flex-start;
    width: 100%;
    height: 100%;
    padding-top: 5px;
    padding-bottom: 5px;
    overflow: auto; 
}

.empty_container {
    padding-top: 20%;
}
</style>