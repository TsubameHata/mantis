<script setup lang="ts">
const total = ref(1);
const current = ref(1);
</script>

<template>
<a-pagination simple
    :total="total"
    v-model:current="current"></a-pagination>
</template>

<style scoped>
</style>