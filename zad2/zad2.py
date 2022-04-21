from multipleAccumulate import MultipleAccumulate
import sys


def sum(x, y):
    return x + y


def product(x, y):
    return x * y


def Accumulate(data_list, *accumulate_functions) -> MultipleAccumulate:
    for function in accumulate_functions:
        if not callable(function):
            print("Arguments needs to be callable (except first)!")
            return None

    return MultipleAccumulate(data_list, *accumulate_functions)


def test():
    accumulate = Accumulate([1, 2, 3, 4, 5], sum,
                            (lambda x, y: max(x, y)), product, (lambda x, y: min(x, y)))

    if accumulate == None:
        return

    viewer_creator = ViewerCreator()
    viewer_creator = viewer_creator.create_viewer("wejscie.txt")

    if viewer_creator == None:
        return

    # viewer_creator.view()

# Duck typing
    for data_getter in [accumulate, viewer_creator]:
        print(data_getter.get_data())


if __name__ == "__main__":
    sys.path.insert(
        0, '/Users/szymon/Documents/Uczelnia/sem4/Jezyki skryptowe/Lista4_Szymon_Sawczuk/zad1')
    from textViewer import TextViewer
    from viewerCreator import ViewerCreator
    test()
