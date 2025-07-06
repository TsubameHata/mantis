<script setup lang="ts">
import scoreStage from './score-stage.vue';

import { useZoomLevel } from '../../store/appState';
import { useActivatedTool } from '../../store/appState';

import { useImgGeometry, useImgSrc } from '../../store/documentState';
import { storeToRefs } from 'pinia';

const zoomLevel = useZoomLevel();

const {activatedTool} =storeToRefs(useActivatedTool());

// Placeholder from IMSLP
const placeholder = "https://cdn.imslp.org/images/thumb/pdfs/75/44210fe8ffe593f14d5a9d65ec8233cb0c904c0b.png";

const imgSrc = useImgSrc();
onMounted(()=>imgSrc.imgSrc = placeholder)

const imgGeometry = useImgGeometry();

const resetZoomRate = (width:number)=>{
    const container = document.getElementById("score_container");
    const container_w = container?.offsetWidth;
    if (container_w) {
        const imgOffsetWidth = container_w * 0.8;
        const zoomRatio = imgOffsetWidth / width;
        const zoomRatioPercent = Math.floor(zoomRatio*100);
        zoomLevel.zoomLevelPercent = zoomRatioPercent;
    };
};

onMounted(()=>{
    const imgElement = document.getElementById("score_page") as HTMLImageElement | null;
    imgElement?.addEventListener("load", ()=>{
        // Cast img geometry to store
        const width = imgElement?.width;
        const height = imgElement?.height;
        if(width && height) imgGeometry.setImgGeometry(width, height);

        resetZoomRate(width);
    });
    window.addEventListener("resize", ()=>{
        const width = imgElement?.width;
        if(width) resetZoomRate(width);
    });
});

const showMarginStage = computed(()=>{
    return activatedTool.value=="margin" || activatedTool.value=="div" || activatedTool.value=="detect"
})
</script>

<template>
<div class="score_container">
<margin-displayer
    v-if="showMarginStage"
    :z-index="50"
    ></margin-displayer>
<division-displayer
    v-if="activatedTool=='div'"></division-displayer>
<img class="score_page" id="score_page"
    :src="imgSrc.imgSrc"/>
</div>
</template>

<style scoped>
.score_container {
    position: relative;
    zoom: v-bind(zoomLevel.zoomRatio);
}
#score_page {
    pointer-events: none;
}
</style>