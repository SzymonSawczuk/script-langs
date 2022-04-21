class TextStats:
    def __init__(self, number_of_lines = 0, number_of_words = 0, number_of_nonalpha =0) -> None:
        self.number_of_lines = number_of_lines
        self.number_of_words = number_of_words
        self.number_of_nonalpha = number_of_nonalpha

    def __count_number_of_nonalpha(self, text) -> int:
        result = 0
        for character in text:
            if not character.isalnum(): result += 1

        return result

    def compute(self, text) -> None:
        self.number_of_lines = len(text.splitlines())
        self.number_of_words = len(text.split())
        self.number_of_nonalpha = self.__count_number_of_nonalpha(text)
