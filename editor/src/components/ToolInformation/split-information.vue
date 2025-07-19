<script setup lang="ts">
import MaterialSymbolsFolderCheckRounded from 'virtual:icons/material-symbols/folder-check-rounded';
import StreamlineStartup from 'virtual:icons/streamline/startup';
import { useImgSrc, usePage } from '../../store/documentState';
import { storeToRefs } from 'pinia';
import { useActivatedTool, useShouldUploadMask } from '../../store/appState';
import axios from 'axios';

const formState = reactive({
    output_img_y: 1080,
    output_img_x: 1920,
    shrink_x_overflow: true,
    shrink_y_overflow: false
});

const saveAllMasks = async ()=>{
    const { pageCount, openedPage } = storeToRefs(usePage());
    const { umTrigger } = useShouldUploadMask();
    const { activatedTool } = storeToRefs(useActivatedTool());

    activatedTool.value = "mask";
    openedPage.value = 1;
    await new Promise(resolve=>{
        setTimeout(resolve, 200);
    });
    umTrigger();
    while(openedPage.value<pageCount.value){
        console.log(openedPage.value)
        openedPage.value++;
        await new Promise(resolve=>{
            setTimeout(resolve, 200);
        });
    };
    umTrigger();
    await new Promise(resolve=>{
        setTimeout(resolve, 200);
    });

    activatedTool.value = "split";
};

const { sessionId } = storeToRefs(useImgSrc());

const processing = ref<boolean>(false);

const processAllMasks = async ()=>{
    processing.value = true;
    await axios.post(`/api/session/${sessionId.value}/process`, {
        output_img_size: [formState.output_img_y, formState.output_img_x],
        shrink_x_overflow: formState.shrink_x_overflow,
        shrink_y_overflow: formState.shrink_y_overflow
    });
    processing.value = false;
}
</script>

<template>
<div class="split_information_container">
<a-card
    hoverable
    size="small">
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
    </a-form>
    <div class="submitBtn_container">
        <a-button type="primary" style="width: 80%;" @click="processAllMasks">
            <template #icon>
                <StreamlineStartup></StreamlineStartup>
            </template>
            {{ $t("split.split") }}</a-button>
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