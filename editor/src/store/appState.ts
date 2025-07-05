import { defineStore } from "pinia";

export const useZoomLevel = defineStore('zoomLevel', ()=>{
    const zoomLevelPercent = ref(100);
    const zoomRatio = computed(()=>zoomLevelPercent.value/100);
    const updateZoomLevelPercent = (newValue:number)=>{
        zoomLevelPercent.value = newValue;
    };
    return {zoomLevelPercent, zoomRatio, updateZoomLevelPercent};
});

export type tool = "cursor" | "margin" | "div" | "detect" | "mask";

export const useActivatedTool = defineStore('activatedTool', ()=>{
    const activatedTool = ref<tool>("cursor");
    const setActivatedTool = (value:tool)=>{
        activatedTool.value = value;
    };
    return {activatedTool, setActivatedTool};
})