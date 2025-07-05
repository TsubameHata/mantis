<script setup lang="ts">
import ScoreDisplayer from './ScoreDisplayer/score-displayer.vue';
</script>

<template>
    <div id="score_container">
        <score-displayer></score-displayer>
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
</style>