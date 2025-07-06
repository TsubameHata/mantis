<script setup lang="ts">
import MaterialSymbolsDeleteOutline from "virtual:icons/material-symbols/delete-outline";
import SolarPenLinear from 'virtual:icons/solar/pen-linear';

import { useDivLines, useMargin } from "../../store/appState";
import { storeToRefs } from "pinia";

import colors from "../../colors";

const {divLines} = useDivLines();
const {top,bottom_h} = storeToRefs(useMargin());

const rgb = (c:number[])=>`rgb(${c[0]},${c[1]},${c[2]})`;
</script>

<template>
<div class="division_information_container">
<a-card 
    hoverable
    size="small" 
    style="width:80%" 
    v-for="(line, index) in divLines">
    <div class="card_content">
        <a-tag
            :color="rgb(colors[index])">{{ index }}</a-tag>
        <a-input-number
            :min="top"
            :max="bottom_h"
            v-model:value="divLines[index]"></a-input-number>
    </div>

    <template #actions>
        <SolarPenLinear></SolarPenLinear>
        <MaterialSymbolsDeleteOutline
            @click="divLines.splice(index, 1)"></MaterialSymbolsDeleteOutline>
    </template>
</a-card>
</div>
</template>

<style scoped>
.division_information_container {
    display: flex;
    align-items: center;
    flex-direction: column;
}
.division_information_container>* {
    margin: 1em 0;
}
.card_content {
    display: flex;
    align-items: center;
    flex-direction: row;
}
.card_content>* {
    margin: auto;
}
</style>