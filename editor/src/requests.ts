
import axios from "axios";

export const new_session = ()=>{
    const input = document.createElement("input");
    input.type = "file";
    input.addEventListener("change", async (e: Event)=>{


        const file = (e.target as HTMLInputElement).files?.[0];
        if(!file) return;
        try {
            const create_session_req = new FormData()
            create_session_req.append("pdfname", file.name)
            const create_session_res = await axios.post("/api/session/create", create_session_req);
            const session_id = create_session_res.data;
            console.log(session_id);

            const import_pdf_req = new FormData();
            import_pdf_req.append("pdf", file);
            const import_pdf_res = await axios.post(`/api/session/${session_id}/pdf`, import_pdf_req);

            
        } catch(error){};
    });
    input.click();
}