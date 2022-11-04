from .video_downloader import VideoDownloader
import requests
import uuid
import ffmpeg

class StaticVideoDownloader(VideoDownloader):
    __THUMBNAIL = "https://www.instandngs4p.eu/wp-content/themes/fox/images/placeholder.jpg"
    def __init__(self, web_url, video_url) -> None:
        super().__init__(web_url)

        self.__video_url = video_url
    
    def get_info(self) -> str:
        meta_data = ffmpeg.probe(self.__video_url)["streams"]
        title = self.__video_url.split("/")[-1]
        duration = int(float(meta_data[0]["duration"]))
        width = meta_data[0]["width"]
        height = meta_data[0]["height"]
        resolutions = [f"{width}x{height} mp4"]
        print({"title": title, "thumbnail": self.__THUMBNAIL, "duration": duration, "resolutions": resolutions})

        return {"title": title, "thumbnail": self.__THUMBNAIL, "duration": duration, "resolutions": resolutions}

    def download_video(self, resolution, time_to_end) -> str:
        r = requests.get(self.__video_url, stream=True)

        path = str(self.BASE_DIR) + "/video_files"
        path_to_result = f"{path}/{uuid.uuid4()}.mp4"

        with open(path_to_result, "wb") as f:
            for chunk in r.iter_content(chunk_size = 1024*1024): 
                if chunk: 
                    f.write(chunk)  

        return path_to_result           

