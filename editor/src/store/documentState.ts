import { defineStore, storeToRefs } from "pinia";
import { usePositiveNumber } from "./utils";

export const usePage = defineStore("page", ()=>{
    const pageCount = usePositiveNumber(0);
    const openedPage = usePositiveNumber(0);
    return {pageCount, openedPage}
})

export const useImgSrc = defineStore('imgSrc', ()=>{
    const openedPage = storeToRefs(usePage()).openedPage;
    // Example: `/api/session/${session_id}/page/`
    const pageURLPrefix = ref("");
    const imgSrc = computed(()=>{
        return pageURLPrefix.value + openedPage.value.toString();
    });
    return {pageURLPrefix, imgSrc};
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