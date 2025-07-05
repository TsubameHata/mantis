<script setup lang="ts">
import marginInformation from './ToolInformation/margin-information.vue';

import { useActivatedTool } from '../store/appState';
import { storeToRefs } from 'pinia';

const {activatedTool} = storeToRefs(useActivatedTool());
</script>

<template>
<div id="information_container">
    <margin-information
        v-if="activatedTool=='margin'"></margin-information>
</div>
</template>

<style scoped>
#information_container {
    width: 100%;
    height: 100%;
    padding: 2em;
}
</style>