from .video_downloader import VideoDownloader
from streamlink import Streamlink
from subprocess import check_output
import yt_dlp
import uuid

class LiveVideoDownloader(VideoDownloader):
    def __init__(self, web_url) -> None:
        super().__init__(web_url)

        self.__ydl  = yt_dlp.YoutubeDL({"outtmpl": self.__generate_path()})

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
            
        temp_resolutions = [] 
        for format in video["formats"]:
            temp_resolutions.append(format["resolution"])


        resolutions = []
        resolutions.append(f"max: {temp_resolutions[len(temp_resolutions) - 2]} mp4")
        resolutions.append(f"min: {temp_resolutions[0]} mp4")

        return {"title": video["title"], "thumbnail": video["thumbnail"], "duration": 0, "resolutions": resolutions}    

    def __stream_to_url(self, url, quality="best"):
        session = Streamlink()
        streams = session.streams(url)
        print(streams[quality].to_url())
        return streams[quality].to_url()

    def download_video(self, resolution, time_to_end) -> str:
        width = resolution.split(" ")[1].split("x")[0]
        height = resolution.split(" ")[1].split("x")[1]

        path_to_result = self.__generate_path()
        stream_url = self.__stream_to_url(self.web_url)
        check_output(["ffmpeg", "-i", stream_url, "-vf", f"scale={width}:{height}", "-ss", "0", "-t", f"{time_to_end * 60}", "-crf", "18", f"{path_to_result}.mp4"])
        
        return f"{path_to_result}.mp4"

            
    
                    

