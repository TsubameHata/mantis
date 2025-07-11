import { defineStore, storeToRefs } from "pinia";

import { usePositiveNumber } from "./utils";

import { useImgGeometry, useMargin } from "./documentState";

export const useZoomLevel = defineStore('zoomLevel', ()=>{
    const zoomLevelPercent = ref(30);
    const zoomRatio = computed(()=>zoomLevelPercent.value/100);
    const resetZoomRate = (width:number)=>{
        const container = document.getElementById("score_container");
        const container_w = container?.offsetWidth;
        if (container_w) {
            const imgOffsetWidth = container_w * 0.8;
            const zoomRatio = imgOffsetWidth / width;
            const zoomRatioPercent = Math.floor(zoomRatio*100);
            zoomLevelPercent.value = zoomRatioPercent;
        };
    };
    return {zoomLevelPercent, zoomRatio, resetZoomRate};
});

export type tool = "cursor" | "margin" | "div" | "detect" | "mask";

export const useActivatedTool = defineStore('activatedTool', ()=>{
    const activatedTool = ref<tool>("cursor");
    return {activatedTool};
});

export const useMargin_ = defineStore("margin", ()=>{
    const left = usePositiveNumber(5);
    const right = usePositiveNumber(5);
    const top = usePositiveNumber(5);
    const bottom = usePositiveNumber(5);

    const {height,width} = storeToRefs(useImgGeometry());

    const bottom_h = computed({
        get: ()=>height.value-bottom.value,
        set: (n:number)=>{
            bottom.value = height.value - n
        }
    });

    const right_w = computed({
        get: ()=>width.value-right.value,
        set: (n:number)=>{
            right.value = width.value - n
        }
    });
    return {
        left,right,top,bottom,bottom_h,right_w
    };
});

export const useDivLines = defineStore("divLines", ()=>{
    const divLines = reactive<number[]>([]);
    const {top, bottom_h} = storeToRefs(useMargin()).openedPageMar.value.margin;
    const divBlocks = computed(()=>{
        if(divLines[0]==undefined) {
            return [[top.value, bottom_h.value]];
        };
        let blocks: number[][] = [];
        blocks.push([top.value, divLines[0]]);
        for(let i=0; i<divLines.length-1; i++){
            blocks.push([divLines[i], divLines[i+1]]);
        };
        blocks.push([divLines[divLines.length-1], bottom_h.value]);
        return blocks;
    });
    return { divLines,divBlocks }
});

export const useMaskBrush = defineStore("maskBrush", ()=>{
    const brushActivated = ref(false);
    const brushRadius = ref(20);
    const nowEditing = ref(0);

    return {brushActivated, brushRadius, nowEditing};
});

export const useMaskLines = defineStore("maskLines", ()=>{
    const maskLines = reactive<{
        index: number,
        points: number[],
        width: number
    }[]>([]);
    
    return {maskLines}
})