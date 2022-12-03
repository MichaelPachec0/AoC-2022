from functools import reduce

max_cal = 0
max_elf = 0
current_elf = 0
current_cal = 0
elves_cal = []
elves_top_three = {"max": 0, "mid": 0, "min": 0}


def work():
    global max_cal, max_elf, elves_cal

    with open("../input.txt", "r") as f:
        for line in f:
            handle_elves(line)
        # input file is not newline terminated, terminate the last elf so that they counted
        handle_elves("\n")
    print(
        f"There was {len(elves_cal)} elves, with the elf bring the most calories is elf #{max_elf + 1} with {max_cal} calories")
    print(f"The answer for Day 1 part 1 is {max_cal}")
    top_3 = reduce(lambda a, b: a+b, [val for _, val in elves_top_three.items()])
    print(f"The combination of the top three elves' calories is {top_3} calories")
    print(f"The answer for Day 1 part 2 is {top_3}")


def handle_elves(line: str):
    global current_elf, current_cal, max_elf, max_cal, elves_cal, elves_top_three
    if len(line) == 1:
        elves_cal.append(current_cal)
        print(f"CURRENT: ELF #{current_elf}: CAL {current_cal}")
        if current_cal > max_cal:
            max_cal = current_cal
            max_elf = current_elf
        for key in elves_top_three.keys():
            if current_cal > elves_top_three[key]:
                handle_top(current_cal, key)
                break
        current_cal = 0
        current_elf = current_elf + 1
    else:
        current_cal = current_cal + int(line)


def handle_top(val: int, cond: str):
    global elves_top_three
    match cond:
        case "max":
            tmp = elves_top_three["max"]
            elves_top_three["max"] = val
            tmp_mid = elves_top_three["mid"]
            elves_top_three["mid"] = tmp
            elves_top_three["min"] = tmp_mid
        case "mid":
            tmp = elves_top_three["mid"]
            elves_top_three["mid"] = val
            elves_top_three["min"] = tmp
        case _:
            elves_top_three["min"] = val


if __name__ == '__main__':
    work()
