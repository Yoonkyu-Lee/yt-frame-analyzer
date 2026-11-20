#include "common.h"
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

/* I1: set access/modification time of PATH to now; create it empty if missing; never truncate. */
static int touch(const char *path) {
    if (utimensat(AT_FDCWD, path, NULL, 0) == 0) return 0;
    if (errno != ENOENT) { error_path("cannot set timestamps", path); return -1; }
    int fd = open(path, O_WRONLY | O_CREAT | O_NONBLOCK, 0666);
    if (fd < 0) { error_path("cannot create", path); return -1; }
    /* PATH may have appeared since utimensat: set its times through the descriptor */
    int rc = futimens(fd, NULL);
    close(fd);
    if (rc < 0) { error_path("cannot set timestamps", path); return -1; }
    return 0;
}

/* C1, C2: every operand, report and continue, usage when none. */
int main(int argc, char **argv) {
    init_program(argv[0]);
    int i = first_operand(argc, argv);
    if (i == argc) { printf("Usage: %s FILE...\n", program); return 1; }
    int failed = 0;
    for (; i < argc; ++i)
        if (touch(argv[i]) < 0) failed = 1;
    return failed;
}
