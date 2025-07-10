<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useDivLines } from '../../store/appState';
import { createDiv, useDiv, usePage } from '../../store/documentState';
import scoreStage from './score-stage.vue';

const {zIndex=51} = defineProps<{zIndex?:number}>();

const {openedPageDiv} = storeToRefs(useDiv());

const stageClicked = (e:any)=>{
    const y = Math.floor(e.target.getStage().getPointerPosition().y);
    openedPageDiv.value.div.divLines.value.push(y);
    openedPageDiv.value.div.divLines.value.sort((a,b)=>a-b);
}
</script>

<template>
<score-stage :z-index="zIndex" @click="stageClicked">
    <v-layer>
        <score-line v-for="(_,index) in openedPageDiv.div.divLines.value"
            v-model:height="openedPageDiv.div.divLines.value[index]"
            stroke="rgb(0,0,255)"></score-line>
    </v-layer>
</score-stage>
</template>