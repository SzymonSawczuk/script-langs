from viewerCreator import ViewerCreator
from textViewer import TextViewer


def test(path):
    viewer_creator = ViewerCreator()
    viewer = viewer_creator.create_viewer(path)

    if viewer == None:
        return

    viewer.view()

    if type(viewer) == TextViewer:
        stats = viewer.get_data()
        print(f"In file {path}:")
        print(f"There is {stats.number_of_lines} number of lines")
        print(f"There is {stats.number_of_words} number of words")
        print(
            f"There is {stats.number_of_nonalpha} number of non-alphanumeric characters")


if __name__ == "__main__":
    test("pobrane.png")
    test("wejscie.txt")
