#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <limits.h>
#include <stdlib.h>


int part_1();

int HandleLongToInt(char *value);

int main() {
    int part1 = part_1();
    fprintf(stdout, "The answer for part 1 is %d\n", part1);
}

int part_1() {
    // have to remember that the cwd is AoC-2022/Day-1/c/cmake-build-debug/ and input is
    // 2 levels above this
    const char *const filename = "../../input.txt";
    int max_cal = 0;
    int cur_cal = 0;
    FILE *input = fopen(filename, "r");
    char line[512];
    int tmp;

    while (fgets(line, sizeof(line), input)) {
        tmp = HandleLongToInt(line);
        if (tmp == 0) {
            if (cur_cal > max_cal) {
                max_cal = cur_cal;
            }
            cur_cal = 0;
        }
        cur_cal += tmp;
    }
    if (input != NULL) {
        fclose(input);
    }
    return max_cal;
}


int HandleLongToInt(char *value) {
    char *err;
    long tmp = strtol(value, &err, 10);
    if (strlen(err) != 1 || tmp > INT_MAX || tmp < INT_MIN || errno) {
        // actual errors, we account for the newline in err, which also works nicely for empty
        // lines, assume that elves won't think to carry 0 calorie food (all food contains
        // calories right? then it would not be food right?, right?)
        return 0;
    }
    return (int) tmp;
}
