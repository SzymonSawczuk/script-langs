from fastapi import Depends, FastAPI, Request, Form
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from pathlib import Path
from urllib3 import HTTPResponse
from starlette.responses import FileResponse
from starlette.exceptions import HTTPException

from .dependencies import get_templates, BASE_DIR, AnyForm
from .routers import static_video, live_video

app = FastAPI()

app.mount("/static", StaticFiles(directory=Path(BASE_DIR, "frontend/static")), name="static")

app.include_router(static_video.router)
app.include_router(live_video.router)

@app.get("/", response_class=HTTPResponse)
async def root(request: Request, templates: Jinja2Templates = Depends(get_templates)):
    return templates.TemplateResponse("index.html", {"request": request})

@app.post(
    "/download",
    tags=["download-video"]
)
async def download_video(resolution: str = Form(), downloader_index: int = Form(), time_to_end: float = Form()):
    print(AnyForm.global_locations)
    if not resolution in AnyForm.global_locations[downloader_index].keys():
        location = AnyForm.global_downloader[downloader_index].download_video(resolution, time_to_end)
        AnyForm.global_locations[downloader_index][resolution] = location
    else:
        location = AnyForm.global_locations[downloader_index][resolution]     

    return get_download_video(location)   

@app.post(
    "/download-file", 
    tags=["download-video"]
)
def get_download_video(location: str):
    return FileResponse(location, media_type='application/octet-stream',filename=f"result.{location[-3: ]}")     


@app.exception_handler(HTTPException)
async def http_exception_handler(request, exc):
    if exc.status_code == 404:
        return get_templates().TemplateResponse('404.html', {'request': request})
    elif exc.status_code == 500:
        return get_templates().TemplateResponse('500.html', {
            'request': request,
            'detail': exc.detail
        })
    else:
        return get_templates().TemplateResponse('error.html', {
            'request': request,
            'detail': exc.detail
        })   
