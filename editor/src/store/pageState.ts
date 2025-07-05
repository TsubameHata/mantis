import { defineStore } from "pinia";

export const useImgSrc = defineStore('imgSrc', ()=>{
    const imgSrc = ref("");
    const setImgSrc = (src:string)=>{
        imgSrc.value = src
    };
    return {imgSrc, setImgSrc};
});

export const useImgGeometry = defineStore('imgGeometry', ()=>{
    const width = ref(0);
    const height = ref(0);
    const setImgGeometry = (w:number, h:number)=>{
        width.value = w;
        height.value = h;
    };
    return {width, height, setImgGeometry}
});