<script setup lang="ts">
import { useActivatedTool } from '../store/appState';

const activatedTool = useActivatedTool();
</script>

<template>
    {{ activatedTool.activatedTool }}
</template>

<style scoped>
</style>