<script setup lang="ts">
const width = 1157;
const height = 1600;
</script>

<template>
<v-stage
    :config="{width, height}"
    style="position:absolute;top:0;left:0;opacity:50%;"
    >
    <slot></slot>
</v-stage>
</template>

<style scoped>
</style>