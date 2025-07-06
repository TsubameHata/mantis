<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useMargin } from '../../store/appState';

const margin = useMargin();

const {left, right, top, bottom} = storeToRefs(margin);
</script>

<template>
<div class="margin_form">
    <div class="input_container">
        <div class="input_label">{{ $t("margin.top") }}</div>
        <a-input-number
            v-model:value="top"
            :min="0"></a-input-number>
    </div>
    <div class="input_container">
        <div class="input_label">{{ $t("margin.bottom") }}</div>
        <a-input-number
            v-model:value="bottom"
            :min="0"></a-input-number>
    </div>
    <div class="input_container">
        <div class="input_label">{{ $t("margin.left") }}</div>
        <a-input-number
            v-model:value="left"
            :min="0"></a-input-number>
    </div>
    <div class="input_container">
        <div class="input_label">{{ $t("margin.right") }}</div>
        <a-input-number
            v-model:value="right"
            :min="0"></a-input-number>
    </div>
</div>
</template>

<style scoped>
.margin_form {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-start;
}

.input_container {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    width: 12em;
}

.input_label {
    padding: 0.4rem;
    flex:1;
    text-align: center;
}

.input_label::after {
    content: ": "
}

.ant-input-number {
    max-width: 6em;
}
</style>