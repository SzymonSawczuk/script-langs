from fastapi import Depends, FastAPI, Request, Form
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from pathlib import Path
from urllib3 import HTTPResponse

from .dependencies import get_templates
from .routers import static_video, live_video

app = FastAPI()

BASE_DIR = Path(".").resolve().parent
app.mount("/static", StaticFiles(directory=Path(BASE_DIR, "frontend/static")), name="static")

app.include_router(static_video.router)
app.include_router(live_video.router)

@app.get("/", response_class=HTTPResponse)
async def root(request: Request, templates: Jinja2Templates = Depends(get_templates)):
    return templates.TemplateResponse("index.html", {"request": request})
