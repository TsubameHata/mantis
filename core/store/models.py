import datetime
from typing import Optional
from sqlmodel import Relationship, SQLModel, Field, UniqueConstraint

class Session(SQLModel, table=True):
    id: int|None = Field(default=None, primary_key=True)
    name: str = Field(unique=True)
    created_at: int = Field(default_factory=lambda: int(datetime.datetime.now().timestamp()))
    page_count: int|None = Field(nullable=True)

    pages: list["Page"]|None = Relationship(back_populates="session")
    masks: list["Mask"]|None = Relationship(back_populates="session")
    results: list["Result"]|None = Relationship(back_populates="session")
    
class Page(SQLModel, table=True):
    # id: int|None = Field(default=None, primary_key=True)
    session_id: int = Field(primary_key=True,foreign_key="session.id")
    index: int = Field(primary_key=True)
    content: bytes
    
    session: Session|None = Relationship(back_populates="pages")
    mask: Optional["Mask"] = Relationship(back_populates="page")
    results: list["Result"]|None = Relationship(back_populates="page")

    __table_args__ = (
        UniqueConstraint("session_id", "index", name="uq_session_page"),
    )
    
class Mask(SQLModel, table=True):
    # id: int|None = Field(default=None, primary_key=True)
    session_id: int = Field(primary_key=True, foreign_key="session.id")
    page_id: int = Field(primary_key=True, foreign_key="page.index")
    
    # JSON str
    color_list: str
    margin: str
    div_blocks: str
    
    content: bytes
    
    page: Page|None = Relationship(back_populates="mask")
    session: Session|None = Relationship(back_populates="masks")

    __table_args__ = (
        UniqueConstraint("session_id", "page_id", name="uq_mask_session_page"),
    )

class Result(SQLModel, table=True):
    # id: int|None = Field(default = None, primary_key=True)
    session_id: int = Field(foreign_key="session.id", primary_key=True)
    page_id: int = Field(foreign_key="page.index", primary_key=True)

    split_index: int = Field(primary_key=True)
    content: bytes
    
    session: Session|None = Relationship(back_populates="results")
    page: Page|None = Relationship(back_populates="results")

    __table_args__ = (
        UniqueConstraint("session_id", "page_id", "split_index", name="uq_res_session_page_index"),
    )