import fitz
import io
from typing import Iterable, Literal, List, IO

from . import utils

def load_pdf(pdf: IO)->fitz.Document:
    """Load PDF to fitz.Document."""
    doc = fitz.open(stream=pdf, filetype="pdf")
    return doc

def pdf2imgs(pdf: fitz.Document, pages: (Literal[True]|Iterable[int]|int)=True, dpi=300)->Iterable[io.BytesIO]:
    """Convert PDF to a list of images of selected pages in io.BytesIO in given order. 
    
    If page=True, convert all the pages."""
    
    length = len(pdf)
    pages_id: Iterable[int]
    
    match pages:
        case True:
            pages_id = range(length)
        case int():
            pages_id = (pages,)
        case _:
            pages_id = pages
    
    dst: List[io.BytesIO] = []
    
    for p in pages_id:
        page = pdf.load_page(p)
        pix = page.get_pixmap(dpi=dpi) # type: ignore

        img_bytes = io.BytesIO()
        img_bytes.write(pix.tobytes("png"))
        img_bytes.seek(0)

        dst.append(img_bytes)
        
    return dst

def test(pdf: str):
    """Convert the specified PDF to images in the same directory."""
    doc: fitz.Document
    with open(pdf, "rb") as f:
        doc = load_pdf(io.BytesIO(f.read()))
    imgs = pdf2imgs(doc, True)
    utils.save_imgs(imgs, pdf)