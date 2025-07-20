from os import getcwd
from os.path import join, dirname, abspath

from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

import api.apis

app = FastAPI()

app.mount("/api", api.apis.app)

distdir = join(abspath(getcwd()), "resources", "editor", "dist")

try:
    app.mount("/", StaticFiles(directory=distdir, html=True), name="static")
except RuntimeError:
    print("Running in dev mode, please make sure that vite dev server is running.")