<script setup lang="ts">
import MaterialSymbolsDeleteOutline from "virtual:icons/material-symbols/delete-outline";
import MingcuteFileNewLine from 'virtual:icons/mingcute/file-new-line';
import MaterialSymbolsRefresh from 'virtual:icons/material-symbols/refresh';
import MaterialSymbolsFileOpenOutline from 'virtual:icons/material-symbols/file-open-outline';

import { Session, get_sessions, new_session } from "../../requests";

const sessions = ref<Session[]>([]);

const refresh_sessions = async ()=>{
    sessions.value = await get_sessions();
    sessions.value = sessions.value.reverse();
}

onMounted(refresh_sessions);
</script>

<template>
<div class="session_information_container">
<div class="operations">
<a-button shape="circle" size="large" type="primary" @click="new_session"><template #icon><MingcuteFileNewLine/></template></a-button>
<a-button shape="circle" size="large" @click="refresh_sessions"><template #icon><MaterialSymbolsRefresh/></template></a-button>
</div>
<a-card 
    hoverable
    size="small"
    v-for="s in sessions">

    <div class="card_content">
        <a-typography-title :level="5">{{ s.name }}</a-typography-title>
        <div>{{ $t("session.created_at") }}{{ new Date(s.created_at*1000).toLocaleString() }} </div>
        <div> {{ s.page_count }} {{ $t("session.page_count") }}</div>
    </div>

    <template #actions>
        <MaterialSymbolsFileOpenOutline
            ></MaterialSymbolsFileOpenOutline>
        <MaterialSymbolsDeleteOutline
            ></MaterialSymbolsDeleteOutline>
    </template>
</a-card>
</div>
</template>

<style scoped>
.operations {
    display: flex;
    flex-direction: row;
    width: 100%;
    justify-content: space-between;
}

.session_information_container {
    display: flex;
    align-items: center;
    flex-direction: column;
}
.session_information_container>* {
    margin: 1em 0;
    width: 100%;
}
.card_content {
    display: flex;
    align-items: center;
    flex-direction: column;
}
.card_content>* {
    margin: auto;
}
</style>