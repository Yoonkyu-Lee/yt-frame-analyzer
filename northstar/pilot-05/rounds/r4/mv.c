#include "common.h"
#include <stdio.h>
#include <stdlib.h>

/* I4, C1, C2: SRC DST renames (replacing a file DST); SRC... DIR moves into DIR. */
int main(int argc, char **argv) {
    init_program(argv[0]);
    int first = first_operand(argc, argv);
    if (argc - first < 2) { printf("Usage: %s SOURCE... DESTINATION\n", program); return 1; }
    char *destination = clean_path(argv[argc - 1]);
    struct stat st;
    bool into_dir = stat(destination, &st) == 0 && S_ISDIR(st.st_mode);
    if (argc - first > 2 && !into_dir) {
        fprintf(stderr, "%s: destination must be a directory\n", program); free(destination); return 1;
    }
    int failed = 0;
    for (int i = first; i < argc - 1; ++i) {
        char *source = clean_path(argv[i]);
        char *target = into_dir ? join_path(destination, base_name(source)) : NULL;
        const char *to = target ? target : argv[argc - 1];
        /* operands go to rename as typed: "file/" then fails with ENOTDIR instead of losing data.
           Cross-device moves fail with EXDEV: copying is a non-goal. */
        if (rename(argv[i], to) < 0) { error_path("cannot move to", to); failed = 1; }
        free(source); free(target);
    }
    free(destination);
    return failed;
}
