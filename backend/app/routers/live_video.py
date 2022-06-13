from fastapi import APIRouter, Depends, Request, Form
from fastapi.templating import Jinja2Templates
from urllib3 import HTTPResponse
import time as t

from ..dependencies import get_templates, AnyForm
from starlette.responses import FileResponse

router = APIRouter(
    prefix="/live-video",
    tags=["live-video"],
    responses={404: {"description": "Not found"}},
)

@router.post(
    "/",
    tags=["live-video"],
    responses={403: {"description": "Operation forbidden"}},
    response_class=HTTPResponse
)
async def get_live_video(request: Request, url: str = Form(), templates: Jinja2Templates = Depends(get_templates)):
    downloader = await AnyForm.get_live_downloader_async(url)
    AnyForm.global_downloader = downloader
    return templates.TemplateResponse("live_video.html", {"request": request, "url": downloader.get_info()})

@router.post(
    "/download",
    tags=["static-video"],
    responses={403: {"description": "Operation forbidden"}}
)
async def download_video(request: Request, time: float = Form(), templates: Jinja2Templates = Depends(get_templates)):
    location = AnyForm.global_downloader.download_video(time)
    t.sleep(time * 60)
    return FileResponse(location, media_type='application/octet-stream',filename="result.mp4")    
