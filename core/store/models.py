import datetime
from typing import Optional
from sqlmodel import Relationship, SQLModel, Field

class Session(SQLModel, table=True):
    id: int|None = Field(default=None, primary_key=True)
    name: str
    created_at: int = Field(default_factory=lambda: int(datetime.datetime.now().timestamp()))

    pages: list["Page"]|None = Relationship(back_populates="session")
    masks: list["Mask"]|None = Relationship(back_populates="session")
    results: list["Result"]|None = Relationship(back_populates="session")
    
class Page(SQLModel, table=True):
    id: int|None = Field(default=None, primary_key=True)
    session_id: int = Field(foreign_key="session.id")
    index: int
    content: bytes
    
    session: Session|None = Relationship(back_populates="pages")
    mask: Optional["Mask"] = Relationship(back_populates="page")
    results: list["Result"]|None = Relationship(back_populates="page")
    
class Mask(SQLModel, table=True):
    id: int|None = Field(default=None, primary_key=True)
    session_id: int = Field(foreign_key="session.id")
    page_id: int = Field(foreign_key="page.id", unique=True)
    
    # JSON str
    color_list: str
    margin: str
    div_blocks: str
    
    content: bytes
    
    page: Page|None = Relationship(back_populates="mask")
    session: Session|None = Relationship(back_populates="masks")

class Result(SQLModel, table=True):
    id: int|None = Field(default = None, primary_key=True)
    session_id: int = Field(foreign_key="session.id")
    page_id: int = Field(foreign_key="page.id")

    split_index: int
    content: bytes
    
    session: Session|None = Relationship(back_populates="results")
    page: Page|None = Relationship(back_populates="results")