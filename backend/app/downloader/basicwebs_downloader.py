from .video_downloader import VideoDownloader
import yt_dlp
import uuid

class BasicWebsVideoDownloader(VideoDownloader):
    def __init__(self, web_url) -> None:
        super().__init__(web_url)
        
        self.__ydl = yt_dlp.YoutubeDL({"outtmpl": self.__generate_path()})
    
    def __generate_path(self):
        path = str(self.BASE_DIR) + "/video_files"
        path_to_result = f"{path}/{uuid.uuid4()}"
        return path_to_result

    def get_info(self) -> str:
        with self.__ydl:
            result = self.__ydl.extract_info(
                self.web_url,
                download=False
            )

        if "entries" in result:
            video = result["entries"][0]
        else:
            video = result
        primary_resolutions = ["240p", "360p", "480p", "720p", "1080p"]

        temp_resolutions = []    
        for format in video["formats"]:
            if any(ext in format["format"] for ext in primary_resolutions) or format["format"][0:4] == "http":
                temp_resolutions.append(format["resolution"])
           
            
        resolutions = []
        resolutions.append(f"max: {temp_resolutions[len(temp_resolutions) - 1]} mp4")
        resolutions.append(f"min: {temp_resolutions[0]} mp4")
        resolutions.append("audio only m4a")

        return {"title": video["title"], "thumbnail": video["thumbnail"], "duration": video["duration"], "resolutions": resolutions}    

    def download_video(self, resolution, time_to_end) -> str:
        format = ""
        ext = ""
        print(resolution)
        if resolution == "audio only m4a":
            print("test")
            format = "bestaudio[ext=m4a]/best"
            ext = "m4a"
        elif resolution[0:3] == "min":
            format = "bestvideo*[height<=240][ext=mp4]+bestaudio[ext=m4a]"
            ext = "mp4"
        elif resolution[0:3] == "max":
            format = "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best"
            ext = "mp4"   

        path_to_result = self.__generate_path()
        self.__ydl = yt_dlp.YoutubeDL({"outtmpl": f"{path_to_result}.{ext}", "format": format})
        self.__ydl.download([self.web_url])
        
        return f"{path_to_result}.{ext}"
