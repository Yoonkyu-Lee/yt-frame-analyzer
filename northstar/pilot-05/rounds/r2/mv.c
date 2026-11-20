#include "common.h"
#include <stdio.h>
#include <stdlib.h>

/* I4, C1, C2: SRC DST renames (replacing a file DST); SRC... DIR moves into DIR. */
int main(int argc, char **argv) {
    init_program(argv[0]);
    if (argc < 3) { printf("Usage: %s SOURCE... DESTINATION\n", program); return 1; }
    char *destination = clean_path(argv[argc - 1]);
    struct stat st;
    bool into_dir = stat(destination, &st) == 0 && S_ISDIR(st.st_mode);
    if (argc > 3 && !into_dir) {
        fprintf(stderr, "%s: destination must be a directory\n", program); free(destination); return 1;
    }
    int failed = 0;
    for (int i = 1; i < argc - 1; ++i) {
        char *source = clean_path(argv[i]);
        char *target = into_dir ? join_path(destination, base_name(source)) : xstrdup(destination);
        /* cross-device moves fail here with EXDEV: copying is a non-goal */
        if (rename(source, target) < 0) { error_path("cannot move to", target); failed = 1; }
        free(source); free(target);
    }
    free(destination);
    return failed;
}
