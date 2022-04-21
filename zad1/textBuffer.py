from os import path as check_path


class TextBuffer():
    def __init__(self, text="") -> None:
        self.text = text

    def read_from_file(self, path) -> None:
        if not check_path.isfile(path):
            print("It is not a file!")
            return

        file = open(path, "r")
        self.text = file.read()
        file.close()
