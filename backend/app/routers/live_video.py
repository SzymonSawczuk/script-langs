from fastapi import APIRouter, Depends, Request, Form
from fastapi.templating import Jinja2Templates
from urllib3 import HTTPResponse

from ..dependencies import get_templates

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
    return templates.TemplateResponse("live_video.html", {"request": request, "url": url})