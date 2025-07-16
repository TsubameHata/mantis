import axios from "axios";

import { usePage, useImgSrc, useMasks } from "./store/documentState";
import { storeToRefs } from "pinia";

export type Session = {
    created_at: number,
    id: number,
    name: string,
    page_count: number
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
        try {
            const create_session_req = new FormData()
            create_session_req.append("pdfname", file.name)
            const create_session_res = await axios.post("/api/session/create", create_session_req);
            const session_id = create_session_res.data;

            const import_pdf_req = new FormData();
            import_pdf_req.append("pdf", file);
            const import_pdf_res = await axios.post(`/api/session/${session_id}/pdf`, import_pdf_req);

            const pageCount = import_pdf_res.data;
            storeToRefs(usePage()).pageCount.value = pageCount;
            storeToRefs(useImgSrc()).pageURLPrefix.value = `/api/session/${session_id}/page/`

            storeToRefs(useMasks()).masks.value.value = [];
        } catch(error){
        };
    });
    input.click();
}