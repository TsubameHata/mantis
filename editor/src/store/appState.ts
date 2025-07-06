import { defineStore, storeToRefs } from "pinia";

import { usePositiveNumber } from "./utils";

import { useImgGeometry } from "./documentState";

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
    const divLines = reactive<number[]>([])
    return { divLines }
})