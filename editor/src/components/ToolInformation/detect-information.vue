<script setup lang="ts">
import MaterialSymbolsFolderCheckRounded from 'virtual:icons/material-symbols/folder-check-rounded';
import MaterialSymbolsDeleteOutline from "virtual:icons/material-symbols/delete-outline";
import { storeToRefs } from 'pinia';
import { gaussian_conv } from '../../requests';
import { useDetectResult } from '../../store/appState';
import colors from '../../colors';

const rgb = (c:number[])=>`rgb(${c[0]},${c[1]},${c[2]})`;

const formState = reactive({
    algorithm: "gaussian_conv",
    min_peak_d: 400,
    min_peak_p: 0.05
});

const { divLines:detectedLines, showDetectResult } = storeToRefs(useDetectResult());
const { saveDetectResult } = useDetectResult();

const submit = async ()=>{
    if(formState.algorithm=="gaussian_conv"){
        detectedLines.value = await gaussian_conv(formState.min_peak_d, formState.min_peak_p);
    }
    showDetectResult.value = true;
}
</script>

<template>
<div class="detect_information_container">
<a-card
    hoverable
    size="small">
    <a-form
        class="card_content"
        :label-col="{span:10}"
        :wrapper-col="{span:12}"
        :model="formState">
        <a-form-item
            :label="$t('detect.algorithm')"
            name="algorithm">
            <a-select v-model:value="formState.algorithm">
                <a-select-option value="gaussian_conv">{{ $t("detect.gaussian_conv") }}</a-select-option>
            </a-select>
        </a-form-item>
        <a-form-item
            :label="$t('detect.min_peak_distance')"
            name="min_peak_d">
            <a-input-number :min="1" :max="1000" style="width:100%" v-model:value="formState.min_peak_d"></a-input-number>
        </a-form-item>
        <a-form-item
            :label="$t('detect.min_peak_prominence')"
            name="min_peak_p">
            <a-input-number :min="0.001" :max="1" :step="0.001" style="width:100%" v-model:value="formState.min_peak_p"></a-input-number>
        </a-form-item>
        <a-form-item :wrapper-col="{ offset: 8, span: 6 }">
            <a-button type="primary" @click="submit">{{ $t("detect.detect") }}</a-button>
        </a-form-item>
    </a-form>
</a-card>
<a-card
    hoverable
    size="small"
    v-if="showDetectResult">
    <div class="lines">
        <div class="line" v-for="(l,index) in detectedLines">
            <a-tag
                :color="rgb(colors[index])">{{ index }}</a-tag>
            <div>{{ l }}</div>
            <material-symbols-delete-outline
                class="svg"
                @click="()=>detectedLines.splice(index,1)"></material-symbols-delete-outline>
        </div>
    </div>
    <div class="operations">
        <a-button 
            shape="round"
            size="large"
            type="primary"
            @click="saveDetectResult">
            <template #icon><material-symbols-folder-check-rounded></material-symbols-folder-check-rounded></template>
            {{ $t("detect.save") }}
        </a-button>
    </div>
</a-card>
</div>
</template>

<style scoped>
.detect_information_container {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.detect_information_container>* {
    margin: 1em 0;
    width: 100%;
}

.operations {
    padding: 1em 1em;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.lines {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 1em 1em;
}

.line {
    display: flex;
    flex-direction: row;
    width: 90%;
    justify-content: space-around;
    margin: 1em 1em;
}

.svg:hover {
    color: #1677ff;
}
</style>