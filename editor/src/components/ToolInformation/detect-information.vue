<script setup lang="ts">
import { gaussian_conv } from '../../requests';

const formState = reactive({
    algorithm: "gaussian_conv",
    min_peak_d: 20,
    min_peak_p: 0.03
});

const detectedLines = ref<number[]>();

const submit = async ()=>{
    if(formState.algorithm=="gaussian_conv"){
        detectedLines.value = await gaussian_conv(formState.min_peak_d, formState.min_peak_p);
    }
}
</script>

<template>
<div class="detect_information_container">
<a-card
    hoverable
    size="small">
    <a-form
        class="card_content"
        :label-col="{span:8}"
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
{{ detectedLines }}
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
</style>