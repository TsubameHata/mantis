<script setup lang="ts">
</script>

<template>
<a-layout class="layout">
    <a-layout-header class="header">
        <header-menu></header-menu>
    </a-layout-header>
    <a-layout>
        <a-layout-sider 
            class="sider-left" 
            collapsible><!--Set :trigger to customize trigger-->
            <side-menu></side-menu>
        </a-layout-sider>
        <a-layout-content class="subsublayout">
            <a-layout-content class="main">
                <central-view></central-view>
            </a-layout-content>
            <a-layout-footer class="bottom-toolbar">
                <bottom-bar></bottom-bar>
            </a-layout-footer>
        </a-layout-content>
        <a-layout-sider
            class="sider-right"
            width="400">
            <information></information>
        </a-layout-sider>
    </a-layout>
</a-layout>
</template>

<style scoped>
.layout {
    height: 100%;
    width: 100%;
    margin: 0;
    padding: 0;
}

.header {
    display: flex;
    flex-direction: row;
    background-color: white;
    width: 100%;
    padding: 0;
    margin: 0;
    box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1),
        0 1px 2px -1px rgba(0,0,0,0.1);
    z-index: 100;
}

.sider-left {
    display: flex;
    flex-direction: column;
    background-color: white;
    padding: 0;
    padding-top: 2em;
    z-index: 99;
    box-shadow: 4px 0 6px -1px rgba(0,0,0,0.1),
        1px 0 2px -1px rgba(0,0,0,0.1);
}

.sider-right {
    display: flex;
    background-color: white;
    z-index: 99;
    box-shadow: -4px 0 6px -1px rgba(0,0,0,0.1),
        -1px 0 2px -1px rgba(0,0,0,0.1);
}

.subsublayout {
    height: 100%;
    display: flex;
    flex-direction: column;
}

.main {
    flex: 1;
    justify-content: center;
    align-items: center;
}

.bottom-toolbar {
    width: 100%;
    background-color: white;
    z-index: 98;
    box-shadow: 0 -4px 6px -1px rgba(0,0,0,0.1),
        0 -1px 2px -1px rgba(0,0,0,0.1);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: row;
    position: relative;
}
</style>
