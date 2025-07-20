"""
Provides data validation and persistent multi-session storage.
"""

import os
import platform
import uuid

from sqlmodel import SQLModel, create_engine, Session

import store.models

DEV = True

STORE_DIR = os.path.join(os.path.abspath(os.getcwd()), "temp")

if platform.system()=="Windows" and not DEV:
    local_appdata = os.getenv("LOCALAPPDATA")
    appdata_dir = os.path.join(local_appdata, "mantis")
    if not os.path.exists(appdata_dir):
        os.mkdir(appdata_dir)
    STORE_DIR = os.path.join(appdata_dir, "temp")

if not os.path.exists(STORE_DIR):
    os.mkdir(STORE_DIR)

DB_DIR = os.path.join(STORE_DIR, "data.db")

sqlite_url = f"sqlite:///{DB_DIR}"
engine = create_engine(sqlite_url)
SQLModel.metadata.create_all(engine)

# for testing purpose
if __name__=="__main__":
    session = Session(engine)
    test = models.Session(name=str(uuid.uuid1()))
    session.add(test)
    session.commit()
    session.close()