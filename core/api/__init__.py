"""
Provides web api connecting the web editor through FastAPI.
"""

from fastapi import FastAPI

app = FastAPI()

@app.get("/")
async def read_root():
    return {"msg": "Hello, Mantis!"}