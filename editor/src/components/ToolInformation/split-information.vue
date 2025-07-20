<script setup lang="ts">
import MaterialSymbolsFolderCheckRounded from 'virtual:icons/material-symbols/folder-check-rounded';
import MaterialSymbolsDownload from 'virtual:icons/material-symbols/download';
import StreamlineStartup from 'virtual:icons/streamline/startup';
import { useImgSrc, usePage } from '../../store/documentState';
import { storeToRefs } from 'pinia';
import { useActivatedTool, useShouldUploadMask, useSplitResult } from '../../store/appState';
import axios from 'axios';
import JSZip from 'jszip';
import { saveAs } from 'file-saver';

const formState = reactive<{
    output_img_y: number,
    output_img_x: number,
    shrink_x_overflow: boolean,
    shrink_y_overflow: boolean,
    omit: number[]
}>({
    output_img_y: 1080,
    output_img_x: 1920,
    shrink_x_overflow: true,
    shrink_y_overflow: false,
    omit: []
});
const { pageCount, openedPage } = storeToRefs(usePage());
const omitOptions = computed(()=>{
    return [...Array(pageCount.value)].map((_,index)=>({value: index+1}))
});
const saveAllMasks = async ()=>{
    const { umTrigger } = useShouldUploadMask();
    const { activatedTool } = storeToRefs(useActivatedTool());

    activatedTool.value = "mask";
    openedPage.value = 1;
    await new Promise(resolve=>{
        setTimeout(resolve, 1000);
    });
    umTrigger();
    while(openedPage.value<pageCount.value){
        await new Promise(resolve=>{
            setTimeout(resolve, 1000);
        });
        openedPage.value++;
        umTrigger();
    };
    await new Promise(resolve=>{
        setTimeout(resolve, 1000);
    });
    umTrigger();
    await new Promise(resolve=>{
        setTimeout(resolve, 1000);
    });

    activatedTool.value = "split";
};

const { sessionId } = storeToRefs(useImgSrc());

const { processingSplit:processing, splitResult, showSplitResult } = storeToRefs(useSplitResult());

const processAllMasks = async ()=>{
    processing.value = true;
    showSplitResult.value = false;

    const pagesToProcess = [...Array(pageCount.value)].map((_,index)=>(index+1)).filter((v)=>(-1==formState.omit.indexOf(v)));

    const response = await axios.post(`/api/session/${sessionId.value}/process`, {
        output_img_size: [formState.output_img_y, formState.output_img_x],
        shrink_x_overflow: formState.shrink_x_overflow,
        shrink_y_overflow: formState.shrink_y_overflow,
        page_indices: pagesToProcess
    });
    splitResult.value = response.data;
    processing.value = false;
    showSplitResult.value = true;
}
const getResultURL = (result: [number,number])=>{
    return `/api/session/${sessionId.value.toString()}/page/${result[0].toString()}/result/${result[1].toString()}`;
}
const saveAllImages = async ()=>{
    const zip = new JSZip();
    const imgFolder = zip.folder("images");

    for(const r of splitResult.value){
        const url = getResultURL(r);
        const res = await fetch(url);
        const blob = await res.blob();
        imgFolder?.file(`${r[0].toString()}-${r[1].toString()}.png`, blob);
    };

    const zipped = await zip.generateAsync({type:"blob"});
    saveAs(zipped, "images.zip")
}
</script>

<template>
<div class="split_information_container">
<a-card
    hoverable
    size="small">
    {{ $t("split.warning_save_masks") }}
    <div class="ensure_masks_saved">
        <a-button
            type="primary"
            shape="round"
            @click="saveAllMasks">
            <template #icon><MaterialSymbolsFolderCheckRounded></MaterialSymbolsFolderCheckRounded></template>
            {{ $t("split.upload_all_masks") }}</a-button>
    </div>
</a-card>
<a-card
    hoverable
    size="small"
    style="align-items: center;justify-content: center;">
    <a-form
        class="card_content"
        :label-col="{span:10}"
        :wrapper-col="{span:12}"
        :model="formState">
        <a-form-item
            :label="$t('split.out_y')"
            name="output_img_y">
            <a-input-number :min="100" :max="10000" style="width:100%" v-model:value="formState.output_img_y" :step="10"></a-input-number>
        </a-form-item>
        <a-form-item
            :label="$t('split.out_x')"
            name="output_img_x">
            <a-input-number :min="100" :max="10000" style="width:100%" v-model:value="formState.output_img_x" :step="10"></a-input-number>
        </a-form-item>
        <a-form-item
            :label="$t('split.shrink_y')"
            name="shrink_y_overflow">
            <a-radio-group v-model:value="formState.shrink_y_overflow">
                <a-radio :value="true">{{ $t("split.yes") }}</a-radio>
                <a-radio :value="false">{{ $t("split.no") }}</a-radio>
            </a-radio-group>
        </a-form-item>
        <a-form-item
            :label="$t('split.shrink_x')"
            name="shrink_x_overflow">
            <a-radio-group v-model:value="formState.shrink_x_overflow">
                <a-radio :value="true">{{ $t("split.yes") }}</a-radio>
                <a-radio :value="false">{{ $t("split.no") }}</a-radio>
            </a-radio-group>
        </a-form-item>
        <a-form-item
            :label="$t('split.omit')"
            name="omit">
            <a-select v-model:value="formState.omit"
                mode="multiple"
                style="width: 100%;"
                :options="omitOptions"></a-select>
        </a-form-item>
    </a-form>
    <div class="submitBtn_container">
        <a-button type="primary" style="width: 80%;" @click="processAllMasks"
            :disabled="processing">
            <template #icon>
                <StreamlineStartup></StreamlineStartup>
            </template>
            {{ $t("split.split") }}</a-button>
    </div>
</a-card>
<a-card
    hoverable
    size="small"
    v-if="showSplitResult">
    <div class="ensure_masks_saved">
        <a-button
            type="primary"
            shape="round"
            @click="saveAllImages">
            <template #icon><MaterialSymbolsDownload></MaterialSymbolsDownload></template>
            {{ $t("split.download_all_images") }}</a-button>
    </div>
</a-card>
</div>
</template>

<style scoped>
.split_information_container {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.split_information_container>* {
    margin: 1em 0;
    width: 100%;
}

.ensure_masks_saved {
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: center;
}

.submitBtn_container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}
</style>