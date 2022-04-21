from functools import reduce


class MultipleAccumulate:
    def __init__(self, data_list, *accumulate_functions) -> None:
        self.data_list = data_list
        self.accumulate_functions = accumulate_functions

    def get_data(self) -> dict:
        result = {}
        lambda_counter = 1
        name = ""

        for function in self.accumulate_functions:
            if function.__name__ == "<lambda>":
                name = f"lambda{lambda_counter}"
                lambda_counter += 1
            else:
                name = function.__name__

            result[name] = reduce(function, self.data_list)

        return result
