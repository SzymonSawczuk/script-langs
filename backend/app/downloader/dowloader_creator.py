from .static_video_downloader import StaticVideoDownloader
from .video_downloader import VideoDownloader
from .live_video_downloader import LiveVideoDownloader
from .basicwebs_downloader import BasicWebsVideoDownloader
from requests_html import AsyncHTMLSession
from starlette.exceptions import HTTPException
import mimetypes
import youtube_dl

class DownloaderCreator():
    __avaible_static_webs = ["www.youtube.com", "twitter.com", "vimeo.com", "youtu.be"]
    __avaible_live_webs = ["www.youtube.com", "youtu.be"]
    def __init__(self) -> None:
        pass

    async def __get_video_url(self, asession, web_url):
            video_request = await asession.get(web_url)
            await video_request.html.arender(sleep=1, keep_page=True, scrolldown=1)
            video = video_request.html.find("video", first=True)

            if video == None:
                return ""

            video_url = video.attrs.get("src")
            return video_url

    async def __return_video_url(self, web_url):
        asession = AsyncHTMLSession()
        return await self.__get_video_url(asession, web_url)

    async def __detect_video_type(self, web_url):
        if web_url.split("/")[2] in self.__avaible_static_webs:
            
            ydl = youtube_dl.YoutubeDL()

            with ydl:
                result = ydl.extract_info(
                    web_url,
                    download=False
                )

            if "entries" in result:
                video = result["entries"][0]
            else:
                video = result

            if "is_live" in video.keys() and video["is_live"]:
                return None, "live"
            return BasicWebsVideoDownloader, "none"

        video_url = await self.__return_video_url(web_url)

        if mimetypes.guess_type(video_url)[0] == "video/mp4":
            return StaticVideoDownloader, video_url

        return None, "none"

    async def create_downloader(self, web_url) -> VideoDownloader:
        if not web_url[0:4] == "http": 
            raise HTTPException(status_code=400, detail="Type of url not avaible")

        try:
            type_of_downloader = await self.__detect_video_type(web_url)
        except:
            raise HTTPException(status_code=500, detail="Error occurs while loading the url")

        if type_of_downloader[1] == "live":
            raise HTTPException(status_code=400, detail="Type of video is live, use other option avaible on site")

        if type_of_downloader[0] == None:
            raise HTTPException(status_code=400, detail="Type of video not avaible")

        if type_of_downloader[0] == StaticVideoDownloader:
            return StaticVideoDownloader(web_url, type_of_downloader[1])    

        return type_of_downloader[0](web_url)    

    async def create_live_downloader(self, web_url) -> VideoDownloader:
        if not web_url[0:4] == "http": 
            raise HTTPException(status_code=400, detail="Type of url not avaible")

        if web_url.split("/")[2] in self.__avaible_live_webs:
            ydl = youtube_dl.YoutubeDL()

            try:
                with ydl:
                    result = ydl.extract_info(
                        web_url,
                        download=False
                    )
            except:
               raise HTTPException(status_code=500, detail="Error occurs while loading the url")        

            if "entries" in result:
                video = result["entries"][0]
            else:
                video = result

            if "is_live" in video.keys() and video["is_live"]:
                return LiveVideoDownloader(web_url)

            else: 
                raise HTTPException(status_code=400, detail="Type of video is not live")  
        else:
            raise HTTPException(status_code=400, detail="Type of video not avaible")


