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


def handle_elves(line: str):
    global current_elf, current_cal, max_elf, max_cal, elves_cal, elves_top_three
    if len(line) == 1:
        elves_cal.append(current_cal)
        print(f"CURRENT: ELF #{current_elf}: CAL {current_cal}")
        if current_cal > max_cal:
            max_cal = current_cal
            max_elf = current_elf
        current_cal = 0
        current_elf = current_elf + 1
    else:
        current_cal = current_cal + int(line)


if __name__ == '__main__':
    work()
