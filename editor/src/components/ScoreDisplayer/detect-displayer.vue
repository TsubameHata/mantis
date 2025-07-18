<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useDiv } from '../../store/documentState';
import scoreStage from './score-stage.vue';

const {zIndex=52} = defineProps<{zIndex?:number}>();

const {openedPageDiv} = storeToRefs(useDiv());

</script>

<template>
<score-stage :z-index="zIndex">
    <v-layer>
        <score-line v-for="(_,index) in openedPageDiv.div.divLines.value"
            v-model:height="openedPageDiv.div.divLines.value[index]"
            stroke="rgb(0,0,255)"></score-line>
    </v-layer>
</score-stage>
</template>