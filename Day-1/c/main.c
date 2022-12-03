#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <limits.h>
#include <stdlib.h>


int part_1();

int HandleLongToInt(const char * value);

int main() {
    int part1 = part_1();
    if (part1 == -1) {
        return -1;
    }
    fprintf(stdout, "The answer for part 1 is %d\n", part1);
}

int part_1() {
    // have to remember that the cwd is AoC-2022/Day-1/c/cmake-build-debug/ and input is
    // 2 levels above this
    const char *const filename = "../../input.txt";
    int max_cal = 0;
    int cur_cal = 0;
    char line[512];
    int tmp;

    FILE *input = fopen(filename, "r");


    if (input == NULL) {
        fprintf(stderr, "FILE WAS NOT READ\nERR: %s\n", strerror(errno));
        return -1;
    }

    while (fgets(line, sizeof(line), input)) {
        tmp = HandleLongToInt(line);
        if (tmp == 0) {
            max_cal = (cur_cal > max_cal) * cur_cal + (cur_cal < max_cal) * max_cal;
            cur_cal = 0;
        }
        cur_cal += tmp;
    }

    fclose(input);
    return max_cal;
}


int HandleLongToInt(const char * const value) {
    char *err;
    long tmp = strtol(value, &err, 10);
    int ret = !(strlen(err) != 1 || tmp > INT_MAX || tmp < INT_MIN || errno) *
            (int) tmp;
    return ret;
}
