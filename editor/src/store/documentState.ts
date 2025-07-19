import { defineStore, storeToRefs } from "pinia";
import { usePositiveNumber } from "./utils";
import { useShouldUploadMask } from "./appState";

export const usePage = defineStore("page", ()=>{
    const pageCount = usePositiveNumber(1);

    const {umTrigger} = useShouldUploadMask();

    const openedPage_ = usePositiveNumber(1);
    // Automatic upload mask in mask mode when page changes.
    const openedPage = computed<number>({
        get: ()=>openedPage_.value,
        set: (i:number)=>{
            if(i>pageCount.value) i=1;
            umTrigger();
            setTimeout(()=>{
                openedPage_.value = i
            }, 200);
        }
    });

    return {pageCount, openedPage}
})

export const useImgSrc = defineStore("imgSrc", ()=>{
    const openedPage = storeToRefs(usePage()).openedPage;
    // Example: `/api/session/${session_id}/page/`
    const sessionId = ref(0);
    const pageURLPrefix = ref("");
    const imgSrc = computed(()=>{
        return pageURLPrefix.value + openedPage.value.toString();
    });
    return {pageURLPrefix, imgSrc, sessionId};
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

export const createMargin = (l:number=5, r:number=5, t:number=5, b:number=5)=>{
    const left = usePositiveNumber(l);
    const right = usePositiveNumber(r);
    const top = usePositiveNumber(t);
    const bottom = usePositiveNumber(b);

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

    const getMargin = (p:number)=>{
        const v = mars.value.find(m=>m.index==p);
        if(v) return v;
        else {

            let margin: ReturnType<typeof createMargin>

            if(mars.value.length>=1){

                // NOTICE: copyFrom isn't sorted by page index, but by the order added.
                // Designed logic not fully implemented.
                let copyFrom = 0;
                if(mars.value.length>=2) copyFrom = mars.value.length - 2;
                else copyFrom = mars.value.length - 1;

                const v = mars.value[copyFrom];

                if(v){
                    margin = createMargin(
                        v.margin.left.value,
                        v.margin.right.value,
                        v.margin.top.value,
                        v.margin.bottom.value
                    );
                } else {
                    margin = createMargin();
                };
            } else {
                margin = createMargin();
            };

            mars.value.push({
                index: openedPage.value,
                margin
            });

            return mars.value[mars.value.length-1]
        }
    }

    const openedPageMar = computed({
        // CAUTION
        get: ()=>getMargin(openedPage.value),
        set: (v:Margin)=>{
            const i = mars.value.findIndex(m=>m.index==openedPage.value);
            if(i!=-1) mars.value[i] = v;
            else mars.value.push(v);
        }
    });

    const { sessionId } = storeToRefs(useImgSrc());
    watch(sessionId, ()=>{
        mars.value = [];
    });

    return { mars, openedPageMar, getMargin }
});

export const createDiv = ()=>{
    const divLines = ref<number[]>([]);
    const { openedPageMar } = storeToRefs(useMargin());
    const divBlocks: ComputedRef<[number,number][]> = computed(()=>{
        if(divLines.value[0]==undefined) {
            return [[openedPageMar.value.margin.top.value, openedPageMar.value.margin.bottom_h.value]];
        };
        let blocks: [number,number][] = [];
        blocks.push([openedPageMar.value.margin.top.value, divLines.value[0]]);
        for(let i=0; i<divLines.value.length-1; i++){
            blocks.push([divLines.value[i], divLines.value[i+1]]);
        };
        blocks.push([divLines.value[divLines.value.length-1], openedPageMar.value.margin.bottom_h.value]);
        return blocks;
    });
    return { divLines,divBlocks }
};

type Div = {
        index: number,
        div: {
            divLines: Ref<number[]>,
            divBlocks: ComputedRef<[number,number][]>
        }
    }

export const useDiv = defineStore("div", ()=>{
    const divs = shallowRef<Div[]>([]);

    const {openedPage} = storeToRefs(usePage());

    const getDiv = (p:number)=>{
        const v = divs.value.find(d=>d.index===p);
        if(v) return v;
        else {
            divs.value.push({
                index: p,
                div: createDiv()
            });
            return divs.value[divs.value.length-1]
        }
    }

    const openedPageDiv = computed({
        // CAUTION: Side effect exists.
        get: ()=>getDiv(openedPage.value),
        set: (newDiv: Div)=>{
            const i = divs.value.findIndex(d=>d.index==openedPage.value);
            if(i!=-1) divs.value[i] = newDiv;
            else divs.value.push(newDiv);
        }
    });

    const { sessionId } = storeToRefs(useImgSrc());
    watch(sessionId, ()=>{
        divs.value = [];
    });

    return { divs,openedPageDiv, getDiv };
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

    const { openedPageDiv } = storeToRefs(useDiv());

    const getMask = (p:number)=>{
        // Ensure div exists.
        const _ = openedPageDiv.value;

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
    }

    const openedPageMask = computed({
        // CAUTION: Side effect exists.
        get: () => getMask(openedPage.value),
        set: (newMask: Mask) => {
            const i = masks.value.findIndex(m => m.index == openedPage.value);
            if (i != -1) masks.value[i] = newMask;
            else masks.value.push(newMask);
        }
    });

    const { sessionId } = storeToRefs(useImgSrc());
    watch(sessionId, ()=>{
        masks.value = [];
    });

    return {masks, openedPageMask, getMask};
});