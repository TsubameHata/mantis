<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useDiv } from '../../store/documentState';
import scoreStage from './score-stage.vue';
import { useDetectResult } from '../../store/appState';
import colors from '../../colors';

const {zIndex=52} = defineProps<{zIndex?:number}>();

const {openedPageDiv} = storeToRefs(useDiv());

const { divLines, showDetectResult } = storeToRefs(useDetectResult());
const rgb = (c:number[])=>`rgb(${c[0]},${c[1]},${c[2]})`;

</script>

<template>
<score-stage :z-index="zIndex">
    <v-layer>
        <score-line v-for="(_,index) in divLines"
            v-model:height="divLines[index]"
            :stroke="rgb(colors[index])"
            :draggable="false"
            cursor="pointer"
            @click="()=>divLines.splice(index,1)"></score-line>
    </v-layer>
</score-stage>
</template>