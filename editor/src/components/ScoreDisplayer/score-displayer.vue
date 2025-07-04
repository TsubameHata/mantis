<script setup lang="ts">
import scoreStage from './score-stage.vue';

import { useZoomLevel } from '../../store/appState';

const zoomLevel = useZoomLevel();
</script>

<template>
<div class="score_container">
<score-stage></score-stage>
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