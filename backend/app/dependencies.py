from pathlib import Path
from fastapi import Form
from fastapi.templating import Jinja2Templates
from .downloader.dowloader_creator import DownloaderCreator
from .downloader.video_downloader import VideoDownloader

class AnyForm():
    global_downloader: VideoDownloader

    @staticmethod
    async def get_downloader_async(web_url):
        downloaderCreator = DownloaderCreator()
        downloader = await downloaderCreator.create_downloader(web_url)
        return downloader

    @staticmethod
    async def get_live_downloader_async(web_url):
        downloaderCreator = DownloaderCreator()
        downloader = await downloaderCreator.create_live_downloader(web_url)
        return downloader    


def get_templates():
    BASE_DIR = Path(".").resolve().parent
    return Jinja2Templates(directory=Path(BASE_DIR, "frontend/static"))
    



