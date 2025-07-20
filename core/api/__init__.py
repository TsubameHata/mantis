from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

import api.apis

app = FastAPI()

app.mount("/api", apis.app)

app.mount("/", StaticFiles(directory="editor/dist", html=True), name="static")