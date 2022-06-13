from .video_downloader import VideoDownloader
import youtube_dl

class BasicWebsVideoDownloader(VideoDownloader):
    def __init__(self, web_url) -> None:
        super().__init__(web_url)
        self.__ydl = ydl = youtube_dl.YoutubeDL({'outtmpl': 'result.mp4'})
    
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

    def download_video(self) -> str:
        self.__ydl.download([self.web_url])
        
        return "result.mp4"
