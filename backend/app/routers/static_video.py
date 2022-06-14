from fastapi import APIRouter, Depends, Request, Form
from fastapi.templating import Jinja2Templates
from urllib3 import HTTPResponse

from ..dependencies import AnyForm, get_templates

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
    (downloader, downloader_index) = await AnyForm.get_downloader_async(url)
    download_info = downloader.get_info()
    return templates.TemplateResponse("video.html", {"request": request, "title": download_info["title"],
                                                     "thumbnail": download_info["thumbnail"], 
                                                     "duration_minutes": int(download_info["duration"]/60), "duration_seconds": int(download_info["duration"]%60),
                                                     "resolutions": download_info["resolutions"], "downloader_index": downloader_index}) 
