import { defineStore } from "pinia";
import { usePositiveNumber } from "./utils";

export const useImgSrc = defineStore('imgSrc', ()=>{
    const imgSrc = ref("");
    return {imgSrc};
});

export const useImgGeometry = defineStore('imgGeometry', ()=>{
    const width = usePositiveNumber(0);
    const height = usePositiveNumber(0);
    const setImgGeometry = (w:number, h:number)=>{
        width.value = w;
        height.value = h;
    };
    return {width, height, setImgGeometry}
});