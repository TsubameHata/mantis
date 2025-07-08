from collections.abc import Iterable
import io
import re
from turtle import mode
from typing import IO
import cv2
from sqlmodel import Session,select,delete
from sqlalchemy.orm import selectinload
import json

from ..store import models,engine
from ..img.pdf2img import load_pdf,pdf2imgs
from ..img.mask import remove_background,get_border_height,resize_and_paste
from ..img.page_utils import load_img,crop_horizontal
from ..img.utils import save_imgs

def create_session(pdfname: str) -> int:
    new_session = models.Session(name=pdfname)
    with Session(engine) as s:
        s.add(new_session)
        s.commit()
    with Session(engine) as s:
        results = s.exec(
            select(models.Session).where(models.Session.name==pdfname)
        )
        id = results.first().id
        assert id!=None
        return id

def remove_session(session_id: int) -> None:
    with Session(engine) as s:
        s.exec(
            delete(models.Session).where(models.Session.id==session_id)
        )
        s.exec(
            delete(models.Page).where(models.Page.session_id==session_id)
        )
        s.exec(
            delete(models.Mask).where(models.Mask.session_id==session_id)
        )
        s.exec(
            delete(models.Result).where(models.Result.session_id==session_id)
        )
        s.commit()

def import_pdf(session_id: int, pdf_stream: IO) -> None:
    pdf = load_pdf(pdf_stream)
    imgs = pdf2imgs(pdf)
    pages = []
    for index,i in enumerate(imgs):
        pages.append(
            models.Page(session_id=session_id,
                        index=index,
                        content=i.read())
        )
    with Session(engine) as s:
        s.add_all(pages)
        s.commit()

# For testing purposes only.
def import_page(session_id: int, page_index: int, path: str)->None:
    """Import a page from the specified path to the session."""
    with open(path, "rb") as f:
        content = f.read()
        page = models.Page(
            session_id=session_id,
            index=page_index,
            content=content
        )
        with Session(engine) as s:
            s.add(page)
            s.commit()

def get_pages(session_id: int, 
              pages: (bool|int|Iterable[int]) = True) -> Iterable[bytes]:
    all = False
    pages_id: Iterable[int] = []
    
    match pages:
        case False:
            pass
        case True:
            all=True
        case int():
            pages_id = (pages,)
        case _:
            pages_id = pages

    with Session(engine) as s:
        sel = select(models.Page)
        results: Iterable[models.Page]
        if all:
            results = s.exec(
                sel.where(models.Page.session_id==session_id)
            )
        else:
            results = s.exec(
                sel.where(models.Page.session_id==session_id,
                                             models.Page.id.in_(pages_id))
            )
        return map(lambda x:x.content, sorted(results, key=lambda x: x.index))

def import_mask(session_id: int, 
                page_id: int, 
                color_list: str, 
                margin: str, 
                div_blocks: str, 
                content: IO) -> None:
    mask = models.Mask(
        session_id=session_id,
        page_id=page_id,
        color_list=color_list,
        margin=margin,
        div_blocks=div_blocks,
        content=content.read()
    )
    with Session(engine) as s:
        s.add(mask)
        s.commit()

def process_masks(session_id: int, 
                  pages: (bool|int|Iterable[int]) = True,
                  output_img_size=(1080,1920),
                  shrink_x_overflow=True,
                  shrink_y_overflow=False) -> Iterable[int]:
    # TODO: Calculate it dynamically
    output_inner_height = 700
    
    all = False
    pages_id: Iterable[int] = []
    
    ps: Iterable[models.Page] = []
    results = []
    
    match pages:
        case False:
            pass
        case True:
            all=True
        case int():
            pages_id = (pages,)
        case _:
            pages_id = pages

    with Session(engine) as s:
        s.exec(
            delete(models.Result).where(
                models.Result.session_id==session_id
                and models.Result.page_id.in_(pages_id) if not all else True
            )
        )
        s_pages = select(models.Page)
        if all:
            ps = s.exec(
                s_pages.where(models.Page.session_id==session_id).options(
                    selectinload(models.Page.mask)
                )
            )
        else:
            ps = s.exec(
                s_pages.where(models.Page.session_id==session_id,
                    models.Page.id.in_(pages_id)
                ).options(
                        selectinload(models.Page.mask)
                )
            )
        ps = sorted(ps, key=lambda x: x.index)

    
        for p in ps:
            color_list = json.loads(p.mask.color_list)
            margin = json.loads(p.mask.margin)
            div_blocks = json.loads(p.mask.div_blocks)

            mask = load_img(io.BytesIO(p.mask.content))
            mask = crop_horizontal(mask, margin["left"], margin["right_w"])

            img = load_img(io.BytesIO(p.content))
            img = crop_horizontal(img, margin["left"], margin["right_w"])

            for index,(color,block) in enumerate(zip(color_list, div_blocks)):
                rb = remove_background(img, mask, color)
                border_height = get_border_height(rb)
                out = resize_and_paste(
                    rb,
                    (block[0], block[1]),
                    border_height,
                    output_img_size,
                    output_inner_height,
                    shrink_x_overflow=shrink_x_overflow,
                    shrink_y_overflow=shrink_y_overflow,
                )
                out_bytes = cv2.imencode(".png", out)[1].tobytes()
                result = models.Result(
                    session_id=session_id,
                    page_id=p.id,
                    split_index=index,
                    content=out_bytes
                )
                results.append(result)
            
        for r in results:
            s.add(r)
        s.commit()

        return_ids = []
        for r in results:
            s.refresh(r)
            return_ids.append(r.id)

        return return_ids

def save_results(session_id: int,
                pages: (bool|int|Iterable[int]) = True) -> Iterable[io.BytesIO]:
    all = False
    pages_id: Iterable[int] = []
    
    match pages:
        case False:
            pass
        case True:
            all=True
        case int():
            pages_id = (pages,)
        case _:
            pages_id = pages

    results: Iterable[models.Result]
    
    with Session(engine) as s:
        sel = select(models.Result)
        if all:
            results = s.exec(
                sel.where(models.Result.session_id==session_id)
            )
        else:
            results = s.exec(
                sel.where(models.Result.session_id==session_id,
                                             models.Result.page_id.in_(pages_id))
            )
        results = sorted(results, key=lambda x: (x.page_id, x.split_index))
        r_io = map(lambda x: io.BytesIO(x.content), results)
        save_imgs(r_io, f"./temp/session_{session_id}_results")