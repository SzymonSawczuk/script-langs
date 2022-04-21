from abc import ABC, abstractmethod


class FileViewer(ABC):
    def __init__(self, path) -> None:
        super().__init__()
        self.path = path

    @abstractmethod
    def view(self) -> None:
        pass
