<script setup lang="ts">
import { useDivLines } from '../../store/appState';
import scoreStage from './score-stage.vue';

const {zIndex=51} = defineProps<{zIndex?:number}>();

const {divLines} = useDivLines();

const stageClicked = (e:any)=>{
    const y = Math.floor(e.target.getStage().getPointerPosition().y);
    divLines.push(y);
    divLines.sort((a,b)=>a-b);
}
</script>

<template>
<score-stage :z-index="zIndex" @click="stageClicked">
    <v-layer>
        <score-line v-for="(_,index) in divLines"
            v-model:height="divLines[index]"
            stroke="rgb(0,0,255)"></score-line>
    </v-layer>
</score-stage>
</template>