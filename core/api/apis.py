"""
Provides web api connecting the web editor through FastAPI.
"""

from io import BytesIO
from typing import Any
from fastapi import FastAPI, File, Form, UploadFile
from fastapi.responses import StreamingResponse
from pydantic import BaseModel
import shared.operations as op
import time

import store

app = FastAPI()

@app.post("/session/create", status_code=201)
def create_session(pdfname: str = Form(...)) -> int:
    session_id: int
    try:
        session_id = op.create_session(pdfname)
    except Exception as e:
        session_id = op.create_session(str(int(time.time()))+pdfname)
    return session_id

@app.get("/session")
def get_sessions() -> list[dict[str, Any]]:
    sessions = op.get_sessions()
    return [s.model_dump() for s in sessions]

@app.delete("/session/{session_id}", status_code=204)
def remove_session(session_id: int) -> None:
    op.remove_session(session_id)

@app.post("/session/{session_id}/pdf", status_code=201)
async def import_pdf(session_id: int, pdf:UploadFile = File(...)) -> None:
    pdf = await pdf.read()
    return op.import_pdf(session_id, BytesIO(pdf))

@app.get("/session/{session_id}/page/{page_index}")
def get_page(session_id: int,
              page_index: int) -> StreamingResponse:
    pages = list(op.get_pages(session_id, page_index))
    return StreamingResponse(BytesIO(pages[0]), media_type="image/png")

@app.get("/session/{session_id}/gaussian_conv/{page_index}")
def get_gaussian_conv(session_id: int,
                       page_index: int,
                       min_peak_prominance: float = 0.03,
                       min_peak_distance: int = 20) -> list[int]:
    peaks = map(int, op.gaussian_conv(session_id, page_index, min_peak_prominance, min_peak_distance))
    return list(peaks)

@app.post("/session/{session_id}/mask/{page_index}", status_code=201)
async def import_mask(session_id: int, 
                      page_index: int, 
                      color_list: str = Form(...), 
                      margin: str = Form(...), 
                      div_blocks: str = Form(...), 
                      mask: UploadFile = File(...)) -> None:
    content = await mask.read()
    op.import_mask(session_id, page_index, color_list, margin, div_blocks, BytesIO(content))

class ProcessMasksRequest(BaseModel):
    page_indices: list[int] | bool | int | None = True
    output_img_size: tuple[int, int] = (1080, 1920)
    shrink_x_overflow: bool = True
    shrink_y_overflow: bool = False

@app.post("/session/{session_id}/process")
def process_masks(req: ProcessMasksRequest, session_id: int) -> list[tuple[int,int]]:
    result_ids = list(op.process_masks(
        session_id,
        req.page_indices,
        req.output_img_size,
        req.shrink_x_overflow,
        req.shrink_y_overflow
    ))
    return result_ids

@app.get("/session/{session_id}/page/{page_index}/result/{result_index}")
def get_result(session_id: int, page_index: int, result_index: int) -> bytes:
    result = list(op.get_results(session_id, page_index))[result_index]
    return StreamingResponse(BytesIO(result), media_type="image/png")

@app.get("/session/{session_id}/page/{page_index}/result_count")
def get_result_count(session_id: int, page_index: int) -> int:
    return len(list(op.get_results(session_id, page_index)))