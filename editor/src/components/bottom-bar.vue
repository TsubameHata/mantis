<script setup lang="ts">
import GravityUiMagnifier from 'virtual:icons/gravity-ui/magnifier';

import { useZoomLevel } from '../store/appState';

import { usePage } from "../store/documentState";
import { storeToRefs } from 'pinia';

const {
    pageCount:total,
    openedPage:current
} = storeToRefs(usePage());

const zoomLevel = useZoomLevel()

const pagKey = ref(0);

watch(total, ()=>{
    console.log(total.value);
    pagKey.value++;
})
</script>

<template>
<a-pagination simple
    :total="total"
    :pageSize="1"
    :key="pagKey"
    v-model:current="current"
    class="pagination"></a-pagination>
<div class="slider">
    <gravity-ui-magnifier></gravity-ui-magnifier>
    <div class="slider_inner">
        <a-slider
            :min="1"
            :max="60"
            :step="1"
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