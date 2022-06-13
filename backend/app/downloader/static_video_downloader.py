from .video_downloader import VideoDownloader
import requests
from requests_html import HTMLSession

class StaticVideoDownloader(VideoDownloader):
    def __init__(self, web_url, video_url) -> None:
        super().__init__(web_url)

        self.__video_url = video_url
    
    def get_info(self) -> str:
        return self.__video_url

    def download_video(self) -> str:
        r = requests.get(self.__video_url, stream=True)

        with open("result.mp4", "wb") as f:
            for chunk in r.iter_content(chunk_size = 1024*1024): 
                if chunk: 
                    f.write(chunk)  

        return  "result.mp4"            

