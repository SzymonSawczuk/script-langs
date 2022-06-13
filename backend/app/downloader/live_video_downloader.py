from time import time
from .video_downloader import VideoDownloader
import youtube_dl
from multiprocessing import Process
from threading import Timer, Thread, Event
import asyncio
from streamlink import Streamlink
from subprocess import Popen

class LiveVideoDownloader(VideoDownloader):
    def __init__(self, web_url) -> None:
        super().__init__(web_url)
        self.__ydl = ydl = youtube_dl.YoutubeDL({'outtmpl': 'result.mp4', 'socket_timeout': 60})

    
    def get_info(self) -> str:
        with self.__ydl:
            result = self.__ydl.extract_info(
                self.web_url,
                download=False
            )

        if 'entries' in result:
            video = result['entries'][0]
        else:
            video = result

        return video['title']   

    def __stream_to_url(self, url, quality='best'):
        session = Streamlink()
        streams = session.streams(url)
        print(streams[quality].to_url())
        return streams[quality].to_url()



    def download_video(self, time_to_end) -> str:
        stream_url = self.__stream_to_url(self.web_url)
        Popen(["ffmpeg", "-i", stream_url, "-ss", "0", "-t", f"{time_to_end * 60}", "-c", "copy", 'stream.mp4'])
        
        return "stream.mp4"

            
    
                    

