#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <limits.h>
#include <stdlib.h>
#include <time.h>

int part_1();

int HandleLongToInt(const char *value);

double time_it(int (*action)(int));

int main() {
    double time_func = time_it(&part_1);
    // The function returns time in seconds, convert it microseconds.
    fprintf(stdout, "The time taken to run function is %fμs", time_func * 1.0E6);
    return 0;

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
    fprintf(stdout, "The answer for part 1 is %d\n", max_cal);
    return 0;
}


int HandleLongToInt(const char *const value) {
    char *err;
    long tmp = strtol(value, &err, 10);
    int ret = !(strlen(err) != 1 || tmp > INT_MAX || tmp < INT_MIN || errno) *
              (int) tmp;
    return ret;
}

#ifdef CLOCK_PROCESS_CPUTIME_ID
/* cpu time in the current process */
#define CLOCKTYPE  CLOCK_PROCESS_CPUTIME_ID
#else
/* this one should be appropriate to avoid errors on multiprocessors systems */
#define CLOCKTYPE  CLOCK_MONOTONIC
#endif


double time_it(int (*action)(int)) {
    struct timespec tsi;
    struct timespec tsf;

    clock_gettime(CLOCKTYPE, &tsi);
    action(0);
    clock_gettime(CLOCKTYPE, &tsf);

    double elapsed_s = difftime(tsf.tv_sec, tsi.tv_sec);
    long elapsed_ns = tsf.tv_nsec - tsi.tv_nsec;
    return elapsed_s + ((double) elapsed_ns) / 1.0e9;
}
