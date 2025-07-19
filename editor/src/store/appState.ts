import { defineStore, storeToRefs } from "pinia";

import { useDiv, usePage } from "./documentState";

export const useZoomLevel = defineStore('zoomLevel', ()=>{
    const zoomLevelPercent = ref(30);
    const zoomRatio = computed(()=>zoomLevelPercent.value/100);
    const resetZoomRate = (width:number)=>{
        const container = document.getElementById("score_container");
        const container_w = container?.offsetWidth;
        if (container_w) {
            const imgOffsetWidth = container_w * 0.6;
            const zoomRatio = imgOffsetWidth / width;
            const zoomRatioPercent = Math.floor(zoomRatio*100);
            zoomLevelPercent.value = zoomRatioPercent;
        };
    };
    return {zoomLevelPercent, zoomRatio, resetZoomRate};
});

export type Tool = "cursor" | "margin" | "div" | "detect" | "mask" | "split";

export const useActivatedTool = defineStore('activatedTool', ()=>{
    const activatedTool = ref<Tool>("cursor");
    return {activatedTool};
});

export const useMaskBrush = defineStore("maskBrush", ()=>{
    const brushActivated = ref(false);
    const brushRadius = ref(20);
    const nowEditing = ref(0);

    return {brushActivated, brushRadius, nowEditing};
});

export const useShouldUploadMask = defineStore("shouldUploadMask", ()=>{
    const umSignal = ref(1)
    const umTrigger = ()=>{
        umSignal.value++
    };
    return {umSignal, umTrigger}
});

export const useDetectResult = defineStore("detectResult", ()=>{
    const divLines = ref<number[]>([]);
    const showDetectResult = ref<boolean>(false);

    const {openedPage} = storeToRefs(usePage());
    watch(openedPage, ()=>{
        showDetectResult.value = false;
        divLines.value = [];
    });

    const div = storeToRefs(useDiv());
    const saveDetectResult = ()=>{
        div.openedPageDiv.value.div.divLines.value = [...divLines.value]
        openedPage.value++;
    };

    return { divLines, showDetectResult, saveDetectResult };
});

export const useSplitResult = defineStore("splitResult", ()=>{
    const splitResult = ref<[number,number][]>([]);
    const showSplitResult = ref<boolean>(false);
    const processingSplit = ref<boolean>(false);

    return { splitResult, showSplitResult, processingSplit }
});