<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useMargin } from '../../store/appState';
import { useImgGeometry } from '../../store/documentState';

const margin = useMargin();
const {left, right, top, bottom} = storeToRefs(margin);

const imgGeometry = useImgGeometry();
const {width,height} = storeToRefs(imgGeometry);

const bottom_h = computed(()=>height.value-bottom.value)
const right_w = computed(()=>width.value-right.value)

const marginDisplayerConfig = {
    stroke: "rgba(0,0,0,1)",
    strokeWidth: 3,
    points: [0,0],
    dash: [20,10,3,10]
};

const getHorizontalLineConfigCurried = (v: Ref<number>)=>()=>{
    let config = {...marginDisplayerConfig};
    config["stroke"] = "rgb(255,0,0)";
    config["points"] = [0, v.value, width.value, v.value];
    return config;
};

const topLineConfig = computed(getHorizontalLineConfigCurried(top));
const bottomLineConfig = computed(getHorizontalLineConfigCurried(bottom_h));

const getVerticalLineConfigCurried = (v: Ref<number>)=>()=>{
    let config = {...marginDisplayerConfig};
    config["stroke"] = "rgb(0,255,0)";
    config["points"] = [v.value, 0, v.value, height.value];
    return config;
};

const leftLineConfig = computed(getVerticalLineConfigCurried(left));
const rightLineConfig = computed(getVerticalLineConfigCurried(right_w));
</script>

<template>
<score-stage>
    <v-layer>
        <v-line :config="topLineConfig"></v-line>
        <v-line :config="bottomLineConfig"></v-line>
        <v-line :config="leftLineConfig"></v-line>
        <v-line :config="rightLineConfig"></v-line>
    </v-layer>
</score-stage>
</template>

<style scoped>
</style>