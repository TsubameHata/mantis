<script setup lang="ts">
import { effect } from 'vue';
import { useZoomLevel } from '../../store/appState';
import { useActivatedTool } from '../../store/appState';

import { useImgGeometry, useImgSrc } from '../../store/documentState';
import { storeToRefs } from 'pinia';

const zoomLevel = useZoomLevel();

const {activatedTool} =storeToRefs(useActivatedTool());

const imgSrc = useImgSrc();

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
    const initImg = ()=>{
        if(!imgElement) return;

        // Cast img geometry to store
        const width = imgElement.width;
        const height = imgElement.height;
        if(width && height) imgGeometry.setImgGeometry(width, height);
        console.log(width,height)

        resetZoomRate(width);
    };
    imgElement?.addEventListener("load", initImg);

    window.addEventListener("resize", ()=>{
        const width = imgElement?.width;
        if(width) resetZoomRate(width);
    });
});

const width = storeToRefs(imgGeometry).width

effect(()=>{
    resetZoomRate(width.value)
})

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
<mask-displayer
    :z-index="52"
    v-if="activatedTool=='mask'"></mask-displayer>
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