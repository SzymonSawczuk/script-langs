from fastapi import APIRouter, Depends, Request, Form
from fastapi.templating import Jinja2Templates
from urllib3 import HTTPResponse

from ..dependencies import get_templates, AnyForm

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
    (downloader, downloader_index) = await AnyForm.get_live_downloader_async(url)
    download_info = downloader.get_info()
    return templates.TemplateResponse("live_video.html", {"request": request, "title": download_info["title"],
                                                     "thumbnail": download_info["thumbnail"], 
                                                     "duration_minutes": int(download_info["duration"]/60), "duration_seconds": int(download_info["duration"]%60),
                                                     "resolutions": download_info["resolutions"], "downloader_index": downloader_index}) 
