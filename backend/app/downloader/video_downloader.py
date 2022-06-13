from abc import ABC, abstractmethod

class VideoDownloader(ABC):
    def __init__(self, web_url) -> None:
        super().__init__()
        self.web_url = web_url

    @abstractmethod
    def get_info(self) -> str:
        pass

    @abstractmethod
    def download_video(self) -> None:
        pass    
