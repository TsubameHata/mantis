<script setup>
const i = ref(0)
</script>

<template>
    <div>
        {{ $t("test") }} {{ i }}
    </div>
    <a-button type="primary" @click="i++"> {{ $t("increment") }} </a-button>
</template>

<style scoped>
</style>
