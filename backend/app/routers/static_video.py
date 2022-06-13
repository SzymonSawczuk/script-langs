from turtle import down
from fastapi import APIRouter, Depends, Request, Form
from fastapi.templating import Jinja2Templates
from urllib3 import HTTPResponse

from ..dependencies import AnyForm, get_templates

from starlette.responses import FileResponse

router = APIRouter(
    prefix="/static-video",
    tags=["static-video"],
    responses={404: {"description": "Not found"}},
)

@router.post(
    "/",
    tags=["static-video"],
    responses={403: {"description": "Operation forbidden"}},
    response_class=HTTPResponse
)
async def get_video(request: Request, url: str = Form(), templates: Jinja2Templates = Depends(get_templates)):
    downloader = await AnyForm.get_downloader_async(url)
    AnyForm.global_downloader = downloader
    return templates.TemplateResponse("video.html", {"request": request, "url": downloader.get_info()})
    
@router.post(
    "/download",
    tags=["static-video"],
    responses={403: {"description": "Operation forbidden"}}
)
async def download_video(request: Request, templates: Jinja2Templates = Depends(get_templates)):
    location = AnyForm.global_downloader.download_video()
    return FileResponse(location, media_type='application/octet-stream',filename="result.mp4")