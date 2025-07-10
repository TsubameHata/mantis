import { defineStore, storeToRefs } from "pinia";
import { usePositiveNumber } from "./utils";
import { Reactive } from "vue";
import { useMargin } from "./appState";

export const usePage = defineStore("page", ()=>{
    const pageCount = usePositiveNumber(1);
    const openedPage = usePositiveNumber(1);
    return {pageCount, openedPage}
})

export const useImgSrc = defineStore("imgSrc", ()=>{
    const openedPage = storeToRefs(usePage()).openedPage;
    // Example: `/api/session/${session_id}/page/`
    const pageURLPrefix = ref("");
    const imgSrc = computed(()=>{
        return pageURLPrefix.value + openedPage.value.toString();
    });
    return {pageURLPrefix, imgSrc};
});

export const useImgGeometry = defineStore("imgGeometry", ()=>{
    const width = usePositiveNumber(0);
    const height = usePositiveNumber(0);
    const setImgGeometry = (w:number, h:number)=>{
        width.value = w;
        height.value = h;
    };
    return {width, height, setImgGeometry}
});

const createDiv = ()=>{
    const divLines = reactive<number[]>([]);
    const {top, bottom_h} = storeToRefs(useMargin());
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
};

type div = {
        index: number,
        div: ReturnType<typeof createDiv>
    }

export const useDiv = defineStore("div", ()=>{
    const divs = ref<div[]>([]);

    const {openedPage} = storeToRefs(usePage());

    const openedPageDiv = computed({
        get: ()=>{
            return divs.value.find(d=>d.index===openedPage.value) ?? {
                index: openedPage.value,
                div: createDiv()
            }
        },
        set: (newDiv: div)=>{
            const i = divs.value.findIndex(d=>d.index==openedPage.value);
            if(i!=-1) divs.value[i]=newDiv;
            else divs.value.push(newDiv);
        }
    })

    return { divs };
});

type mask = {
        index: number,
        uri: string,
        lines: {
            index: number,
            points: number[],
            width: number
        }[]
    }

export const useMasks = defineStore("masks", ()=>{
    const masks = ref<mask[]>([]);

    const { openedPage } = storeToRefs(usePage());

    const openedPageMask = computed({
        get: () => {
            return masks.value.find(m => m.index == openedPage.value) ?? {index:openedPage, uri:"", lines: []};
        },
        set: (newMask: mask) => {
            const i = masks.value.findIndex(m => m.index == openedPage.value);
            if (i != -1) masks.value[i] = newMask;
            else masks.value.push(newMask);
        }
    });

    return {masks, openedPageMask};
})