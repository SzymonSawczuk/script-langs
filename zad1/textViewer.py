from fileViewer import FileViewer
from textBuffer import TextBuffer
from textStats import TextStats
from os import path as check_path
import subprocess


class TextViewer(FileViewer, TextBuffer):
    def __init__(self, path) -> None:
        super().__init__(path)
        TextBuffer().__init__()
        self.__stats = TextStats()

        self.read_from_file(self.path)
        self.__stats.compute(self.text)

    def view(self) -> None:
        if not check_path.exists(self.path):
            print("File {self.path} does not exists!")
            return
        subprocess.run(["open", "-a", "TextEdit", self.path])

    def get_data(self) -> TextStats:
        return self.__stats
