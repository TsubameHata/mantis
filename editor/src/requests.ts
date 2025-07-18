import axios from "axios";

import { usePage, useImgSrc, useMasks, useDiv, useMargin } from "./store/documentState";
import { storeToRefs } from "pinia";

import colors from "./colors";

export type Session = {
    created_at: number,
    id: number,
    name: string,
    page_count: number
}

export const set_session = (id:number, pageCount:number)=>{
    // TODO: if masks aren't uploaded, upload them
    const {pageCount:pc, openedPage} = storeToRefs(usePage());
    pc.value = pageCount;
    openedPage.value = 1;
    
    const { pageURLPrefix, sessionId } = storeToRefs(useImgSrc());

    sessionId.value = id;
    pageURLPrefix.value = `/api/session/${id}/page/`

    storeToRefs(useMasks()).masks.value = [];
}

export const get_sessions: ()=>Promise<Session[]> = async ()=>{
    const get_sessions_res = await axios.get("/api/session");
    return get_sessions_res.data;
}

export const new_session = ()=>{
    const input = document.createElement("input");
    input.type = "file";
    input.accept = ".pdf"
    input.addEventListener("change", async (e: Event)=>{

        const file = (e.target as HTMLInputElement).files?.[0];
        if(!file) return;

        const create_session_req = new FormData()
        create_session_req.append("pdfname", file.name)
        const create_session_res = await axios.post("/api/session/create", create_session_req);
        const session_id = create_session_res.data;

        const import_pdf_req = new FormData();
        import_pdf_req.append("pdf", file);
        const import_pdf_res = await axios.post(`/api/session/${session_id}/pdf`, import_pdf_req);
        const pageCount = import_pdf_res.data;
        
        set_session(session_id, pageCount);
    });
    input.click();
}

export const remove_session = async (id:number)=>{
    await axios.delete(`/api/session/${id}`);
}

export const gaussian_conv: (minPeakDistance?:number, minPeakProminence?:number)=>Promise<number[]> = async (minPeakDistance: number = 20, minPeakProminence:number = 0.03)=>{
    const { sessionId } = storeToRefs(useImgSrc());
    const { openedPage } = storeToRefs(usePage());

    const gaussian_conv_res = await axios.get(
        `/api/session/${sessionId.value}/gaussian_conv/${openedPage.value}?min_peak_distance=${minPeakDistance.toString()}&min_peak_prominence=${minPeakProminence.toString()}`
    );
    return gaussian_conv_res.data;
}

export const upload_mask = async (imgBlob: Blob)=>{
    const upload_mask_req = new FormData();
    
    const { sessionId } = storeToRefs(useImgSrc());
    const { openedPage } = storeToRefs(usePage());
    const page_index = openedPage.value;

    const marginRefs = useMargin().getMargin(page_index).margin;
    const margin = {
        top: marginRefs.top.value,
        bottom: marginRefs.bottom.value,
        bottom_h: marginRefs.bottom_h.value,
        left: marginRefs.left.value,
        right: marginRefs.right.value,
        right_w: marginRefs.right_w.value
    };
    upload_mask_req.append("margin", JSON.stringify(margin));

    const div_blocks = useDiv().getDiv(page_index).div.divBlocks.value;
    upload_mask_req.append("div_blocks", JSON.stringify(div_blocks));

    const color_list: [number,number,number][] = colors.slice(0, div_blocks.length);
    upload_mask_req.append("color_list", JSON.stringify(color_list));

    const imgFile = new File([imgBlob], "mask.png", {type:imgBlob.type})
    upload_mask_req.append("mask", imgFile)

    return await axios.post(`/api/session/${sessionId.value}/mask/${page_index}`, upload_mask_req);
}