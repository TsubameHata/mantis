<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useSplitResult } from '../../store/appState';
import { useImgSrc } from '../../store/documentState';

const {sessionId} = storeToRefs(useImgSrc());
const { splitResult,showSplitResult,processingSplit } = storeToRefs(useSplitResult());

const getResultURL = (result: [number,number])=>{
    return `/api/session/${sessionId.value.toString()}/page/${result[0].toString()}/result/${result[1].toString()}`;
}
</script>

<template>
<div class="split_displayer">
<div class="spin_container" v-if="processingSplit">
    <a-spin size="large"></a-spin>
</div>
<div class="result_container" v-if="showSplitResult">
<a-image-preview-group>
<div class="result_img" v-for="r in splitResult">
    <a-image
        :width="200"
        :src="getResultURL(r)"></a-image>
    <div class="description">{{ r[0] }}, {{ r[1] }}</div>
</div>
</a-image-preview-group>
</div>
</div>
</template>

<style scoped>
.split_displayer {
    height: 100%;
    overflow-y: auto;
}

.spin_container {
    display: flex;
    height: 100%;
    align-items: center;
    justify-content: center;
}

.result_container {
    display: flex;
    height: fit-content;
    align-items: center;
    justify-content: center;
    flex-direction: row;
    flex-wrap: wrap;
}

.result_container>* {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    margin: 1em;
}
.result_img>* {
    margin: 0.2em 0;
}
</style>