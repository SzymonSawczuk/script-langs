from fileViewer import FileViewer
from os import path as check_path
import subprocess


class ImageViewer(FileViewer):
    def __init__(self, path) -> None:
        super().__init__(path)

    def view(self) -> None:
        if not check_path.exists(self.path):
            print("File {self.path} does not exists!")
            return
        subprocess.run(["open", "-a", "Google Chrome", self.path])
