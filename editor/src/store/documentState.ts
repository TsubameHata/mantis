import { defineStore, storeToRefs } from "pinia";
import { usePositiveNumber } from "./utils";
import { useMargin_ } from "./appState";

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

type Margin = {
    index: number,
    margin: {
        left: Ref<number>, right: Ref<number>,
        bottom: Ref<number>, top: Ref<number>,
        bottom_h: ComputedRef<number>,
        right_w: ComputedRef<number>
    }
}

export const createMargin = ()=>{
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
};

export const useMargin = defineStore("mar", ()=>{
    const mars = shallowRef<Margin[]>([]);

    const {openedPage} = storeToRefs(usePage());

    const openedPageMar = computed({
        // CAUTION
        get: ()=>{
            const v = mars.value.find(m=>m.index==openedPage.value);
            if(v) return v;
            else {
                mars.value.push({
                    index: openedPage.value,
                    margin: createMargin()
                });

                // if(mars.value.length>=1){
                //     let copyFrom = 0;
                //     if(mars.value.length>=2) copyFrom = mars.value.length - 3;
                //     else copyFrom = mars.value.length - 2;
                //     mars.value[mars.value.length-1].margin.bottom.value = mars.value[copyFrom].margin.bottom.value
                //     mars.value[mars.value.length-1].margin.top.value = mars.value[copyFrom].margin.top.value
                //     mars.value[mars.value.length-1].margin.left.value = mars.value[copyFrom].margin.left.value
                //     mars.value[mars.value.length-1].margin.right.value = mars.value[copyFrom].margin.right.value
                // }
                return mars.value[mars.value.length-1]
            }
        },
        set: (v:Margin)=>{
            const i = mars.value.findIndex(m=>m.index==openedPage.value);
            if(i!=-1) mars.value[i] = v;
            else mars.value.push(v);
        }
    });
    return { mars, openedPageMar }
});

export const createDiv = ()=>{
    const divLines = ref<number[]>([]);
    const {top, bottom_h} = toRefs(storeToRefs(useMargin()).openedPageMar.value.margin);
    const divBlocks = computed(()=>{
        if(divLines.value[0]==undefined) {
            return [[top.value, bottom_h.value]];
        };
        let blocks: number[][] = [];
        blocks.push([top.value, divLines.value[0]]);
        for(let i=0; i<divLines.value.length-1; i++){
            blocks.push([divLines.value[i], divLines.value[i+1]]);
        };
        blocks.push([divLines.value[divLines.value.length-1], bottom_h.value]);
        return blocks;
    });
    return { divLines,divBlocks }
};

type Div = {
        index: number,
        div: {
            divLines: Ref<number[]>,
            divBlocks: ComputedRef<number[][]>
        }
    }

export const useDiv = defineStore("div", ()=>{
    const divs = shallowRef<Div[]>([]);

    const {openedPage} = storeToRefs(usePage());

    const openedPageDiv = computed({
        // CAUTION: Side effect exists.
        get: ()=>{
            const v = divs.value.find(d=>d.index===openedPage.value);
            if(v) return v;
            else {
                divs.value.push({
                    index: openedPage.value,
                    div: createDiv()
                });
                return divs.value[divs.value.length-1]
            }
        },
        set: (newDiv: Div)=>{
            const i = divs.value.findIndex(d=>d.index==openedPage.value);
            if(i!=-1) divs.value[i] = newDiv;
            else divs.value.push(newDiv);
        }
    })

    return { divs,openedPageDiv };
});

type Mask = {
        index: number,
        uri: string,
        lines: {
            index: number,
            points: number[],
            width: number
        }[]
    }

export const useMasks = defineStore("masks", ()=>{
    const masks = ref<Mask[]>([]);

    const { openedPage } = storeToRefs(usePage());

    const openedPageMask = computed({
        // CAUTION: Side effect exists.
        get: () => {
            const v = masks.value.find(m => m.index == openedPage.value);
            if(v) return v;
            else {
                masks.value.push({
                    index: openedPage.value,
                    uri: "",
                    lines: []
                });
                return masks.value[masks.value.length-1]
            }
        },
        set: (newMask: Mask) => {
            const i = masks.value.findIndex(m => m.index == openedPage.value);
            if (i != -1) masks.value[i] = newMask;
            else masks.value.push(newMask);
        }
    });

    return {masks, openedPageMask};
});