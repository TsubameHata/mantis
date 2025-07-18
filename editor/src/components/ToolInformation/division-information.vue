<script setup lang="ts">
import MaterialSymbolsDeleteOutline from "virtual:icons/material-symbols/delete-outline";
import MaterialSymbolsFolderCheckRounded from 'virtual:icons/material-symbols/folder-check-rounded';

import { useActivatedTool, useMaskBrush } from "../../store/appState";
import { storeToRefs } from "pinia";

import colors from "../../colors";
import { useDiv, useMargin, useMasks } from "../../store/documentState";
import { useShouldUploadMask } from "../../store/appState";

const { openedPageDiv:opd } = storeToRefs(useDiv());
const { openedPageMar:opm } = storeToRefs(useMargin());

const { activatedTool } = storeToRefs(useActivatedTool());

const rgb = (c:number[])=>`rgb(${c[0]},${c[1]},${c[2]})`;

const { brushRadius } = storeToRefs(useMaskBrush());

const { umTrigger } = useShouldUploadMask();
</script>

<template>
<div class="division_information_container">
<div class="operations">
<a-button shape="round" size="large" type="primary" 
    @click="umTrigger">
    <template #icon><MaterialSymbolsFolderCheckRounded/></template>
    {{ $t("divisions.save") }}
</a-button>
<a-button shape="round" size="large" @click="useMasks().openedPageMask.lines = []">
    <template #icon><MaterialSymbolsDeleteOutline/></template>
    {{ $t("divisions.delete") }}
</a-button>
</div>
<a-card
    hoverable
    size="small"
    v-if="activatedTool=='mask'">
    <div class="slider_card card_content">
        <div>{{$t("div.brush_r")}}</div>
        <div class="slider_container">
            <div class="slider_wrap">
                <a-slider v-model:value="brushRadius"
                    :min="2" :max="100"></a-slider>
            </div>
            <div class="input_wrap">
                <a-input-number
                    v-model:value="brushRadius"
                    :min="2" :max="100"></a-input-number>
            </div>
            
        </div>
    </div>
</a-card>
<a-card
    hoverable
    size="small">
    <div class="card_content">
    <a-tag
        :color="rgb(colors[0])">0</a-tag>
    <a-input-number
        :disabled="true"
        :value="opm.margin.top.value"></a-input-number>
    </div>
    <template #actions>
        <toggle-pen :index="0"></toggle-pen>
    </template>
</a-card>
<a-card 
    hoverable
    size="small" 
    v-for="(_, index) in opd.div.divLines.value">

    <div class="card_content">
        <a-tag
            :color="rgb(colors[index+1])">{{ index+1 }}</a-tag>
        <a-input-number
            :min="opm.margin.top.value"
            :max="opm.margin.bottom_h.value"
            v-model:value="opd.div.divLines.value[index]"></a-input-number>
    </div>

    <template #actions>
        <toggle-pen :index="index+1"></toggle-pen>
        <MaterialSymbolsDeleteOutline
            @click="opd.div.divLines.value.splice(index, 1)"></MaterialSymbolsDeleteOutline>
    </template>
</a-card>
</div>
</template>

<style scoped>
.operations {
    display: flex;
    flex-direction: row;
    width: 100%;
    justify-content: space-between;
}
.division_information_container {
    display: flex;
    align-items: center;
    flex-direction: column;
}
.division_information_container>* {
    margin: 1em 0;
    width: 100%;
}
.card_content {
    display: flex;
    align-items: center;
    flex-direction: row;
}
.card_content>* {
    margin: auto;
}
.slider_card {
    flex-direction: column;
    text-align: start;
}
.slider_container {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    width: 100%;
}
.slider_wrap {
    padding: 0 0.5em;
    width: 50%;
}
.input_wrap {
    padding: 0 0.5em;
    width: 40%;
}
</style>