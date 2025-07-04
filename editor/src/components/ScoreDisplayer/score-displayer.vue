<script setup lang="ts">
import { useZoomLevel } from '../../store/appState';

const zoomLevel = useZoomLevel();
</script>

<template>
<div class="score_container">
<v-stage
    :config="{width: 1157, height: 1600}"
    style="position:absolute;top:0;left:0;opacity:50%;"
    >
    <v-layer>
        <v-rect
            :config="{
                x:0,
                y:0,
                width:1157,
                height:1600,
                fill: 'rgb(255,0,0)'
            }"></v-rect>
    </v-layer>
</v-stage>
<!-- Placeholder from IMSLP -->
<img class="score_page" id="score_page"
        src="https://cdn.imslp.org/images/thumb/pdfs/75/44210fe8ffe593f14d5a9d65ec8233cb0c904c0b.png"/>
</div>
</template>

<style scoped>
.score_container {
    position: relative;
    zoom: v-bind(zoomLevel.zoomRatio);
}
</style>