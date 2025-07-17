<script setup lang="ts">
import { storeToRefs } from 'pinia';
import ScoreDisplayer from './ScoreDisplayer/score-displayer.vue';
import { useImgSrc } from '../store/documentState';

const { pageURLPrefix } = storeToRefs(useImgSrc());

const showScore = computed(()=>pageURLPrefix.value!="");
</script>

<template>
    <div id="score_container">
        <score-displayer v-if="showScore"></score-displayer>
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