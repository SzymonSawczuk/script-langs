from pathlib import Path
from fastapi.templating import Jinja2Templates
from .downloader.dowloader_creator import DownloaderCreator
from .downloader.video_downloader import VideoDownloader

BASE_DIR = Path(".").resolve().parent
print(BASE_DIR)

class AnyForm():
    global_downloader: list[VideoDownloader] = []
    global_current_size: int = 0
    global_locations: list[dict] = []

    @staticmethod
    async def __helper_get_downloader(web_url, create_downloader):
        downloader = await create_downloader(web_url)

        AnyForm.global_downloader.append(downloader)
        AnyForm.global_locations.append({})
        AnyForm.global_current_size += 1
        return (AnyForm.global_downloader[AnyForm.global_current_size - 1], AnyForm.global_current_size - 1) 

    @staticmethod
    async def get_downloader_async(web_url):
        downloaderCreator = DownloaderCreator()
        return await AnyForm.__helper_get_downloader(web_url, downloaderCreator.create_downloader)

    @staticmethod
    async def get_live_downloader_async(web_url):
        downloaderCreator = DownloaderCreator()
        return await AnyForm.__helper_get_downloader(web_url, downloaderCreator.create_live_downloader)

def get_templates():
    return Jinja2Templates(directory=Path(BASE_DIR, "frontend/static"))
    



