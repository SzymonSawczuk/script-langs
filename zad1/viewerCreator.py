from fileViewer import FileViewer
from imageViewer import ImageViewer
from textViewer import TextViewer
import mimetypes

class ViewerCreator:
    def __init__(self) -> None:
        pass

    def __detect_viewer_type(self, path) -> type:
        test = mimetypes.guess_type(path)
        if test == (None, None):
            print("Non avaible option!")
            return None
            
        match test[0].split("/")[0]:
            case 'text':
                return TextViewer
            case 'image':
                return ImageViewer
            case _:
                print("Non avaible option!")
                return None

    def create_viewer(self, path) -> FileViewer:
        type_of_Viewer = self.__detect_viewer_type(path)
        if type_of_Viewer == None:
            return None
        return type_of_Viewer(path)


        
