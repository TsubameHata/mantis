import { defineStore } from "pinia";

import { usePositiveNumber } from "./utils";

export const useZoomLevel = defineStore('zoomLevel', ()=>{
    const zoomLevelPercent = ref(100);
    const zoomRatio = computed(()=>zoomLevelPercent.value/100);
    return {zoomLevelPercent, zoomRatio};
});

export type tool = "cursor" | "margin" | "div" | "detect" | "mask";

export const useActivatedTool = defineStore('activatedTool', ()=>{
    const activatedTool = ref<tool>("cursor");
    return {activatedTool};
});

export const useMargin = defineStore("margin", ()=>{
    const left = usePositiveNumber(5);
    const right = usePositiveNumber(5);
    const top = usePositiveNumber(5);
    const bottom = usePositiveNumber(5);
    return {
        left,right,top,bottom
    };
});