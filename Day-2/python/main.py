from typing import Iterator


def reader(path: str, delimiter: str = "//") -> Iterator[str]:
    with open(path, "r") as fp:
        for line in fp:
            if len(line) == 0 or delimiter in line:
                continue
            yield line.strip()


def reader_splitter(path: str, splitter: str = ":") -> Iterator[list[str]]:
    for line in reader(path):
        yield line.split(splitter)


def part_1(day_input: list[str], outcomes: dict[str: int]) -> int:
    return sum(outcomes[line] for line in day_input)


def part_2(day_input: list[str], outcomes: dict[str: int], conditions: dict[str: str]) -> int:
    ret = 0
    for line in day_input:
        enemy = line.split(" ")[0]
        choice = conditions[line]
        ret += outcomes[f'{enemy} {choice}']
    return ret


def main():
    day_input = [line for line in reader("../input.txt")]
    outcomes = {line[0]: int(line[1]) for line in reader_splitter("../outcomes.txt")}
    part_1_score = part_1(day_input, outcomes)
    print(f"The score for the player using the guide is {part_1_score}")
    conditions = {line[0]: line[1] for line in reader_splitter("../part2.txt")}
    part_2_score = part_2(day_input, outcomes, conditions)
    print(f"The score for the player with part 2's conditions in mind is {part_2_score}")


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    main()

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
