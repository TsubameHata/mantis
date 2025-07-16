<script setup lang="ts">
import marginInformation from './ToolInformation/margin-information.vue';
import divisionInformation from './ToolInformation/division-information.vue';

import { useActivatedTool } from '../store/appState';
import { storeToRefs } from 'pinia';

const {activatedTool} = storeToRefs(useActivatedTool());

const showDivInformation = computed(()=>{
    return activatedTool.value=='div' || activatedTool.value=='mask'
})
</script>

<template>
<div id="information_container">
    <margin-information
        v-if="activatedTool=='margin'"></margin-information>
    <division-information
        v-if="showDivInformation"></division-information>
    <session-information
        v-if="activatedTool=='cursor'"></session-information>
</div>
</template>

<style scoped>
#information_container {
    width: 100%;
    height: 100%;
    padding: 2em;
    overflow-y: auto;
}
</style>

<style>
.ant-layout-sider-children {
    width: 100%;
}
</style>