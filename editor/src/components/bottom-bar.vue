<script setup lang="ts">
import GravityUiMagnifier from 'virtual:icons/gravity-ui/magnifier';

import { useZoomLevel } from '../store/appState';

const total = ref(1);
const current = ref(1);

const zoomLevel = useZoomLevel()
</script>

<template>
<a-pagination simple
    :total="total"
    v-model:current="current"
    class="pagination"></a-pagination>
<div class="slider">
    <gravity-ui-magnifier></gravity-ui-magnifier>
    <div class="slider_inner">
        <a-slider
            :min="30"
            :max="150"
            v-model:value="zoomLevel.zoomLevelPercent"
            :tipFormatter="i=>`${i}%`"></a-slider>
    </div>
</div>
</template>

<style scoped>
.pagination {
    margin: 0 auto;
}
.slider {
    display: flex;
    flex-direction: row;
    align-items: center;
    width: 160px;
    position: absolute;
    right: 40px;
}
.slider>div {
    padding: 0 10px;
}
.slider_inner {
    width: 140px;
}
</style>