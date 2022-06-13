from abc import ABC, abstractmethod

from pathlib import Path

class VideoDownloader(ABC):
    BASE_DIR = Path(".").resolve().parent
    print(BASE_DIR)

    def __init__(self, web_url) -> None:
        super().__init__()
        self.web_url = web_url

    @abstractmethod
    def get_info(self) -> str:
        pass

    @abstractmethod
    def download_video(self, resolution, time_to_end) -> str:
        pass    
