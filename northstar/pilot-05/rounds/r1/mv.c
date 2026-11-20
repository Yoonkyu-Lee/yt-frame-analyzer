#include "common.h"
#include <errno.h>
#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static void usage(void) {
    printf("Usage: %s [-fnvT] SOURCE... DESTINATION\n"
           "  -f overwrite  -n no overwrite\n"
           "  -v report moves  -T treat destination as a file\n", program);
}
static int move_one(const char *source, const char *target, bool no_clobber, bool verbose) {
    struct stat src, dst;
    if (lstat(source, &src) < 0) { error_path("cannot stat", source); return -1; }
    int exists = lstat(target, &dst) == 0;
    if (!exists && errno != ENOENT) { error_path("cannot stat", target); return -1; }
    if (exists) {
        if (src.st_dev == dst.st_dev && src.st_ino == dst.st_ino) {
            fprintf(stderr, "%s: '%s' and '%s' are the same file\n", program, source, target); return -1;
        }
        if (no_clobber) return 0;
    }
    if (rename(source, target) == 0) {
        if (verbose) printf("'%s' -> '%s'\n", source, target);
        return 0;
    }
    error_path("cannot move to", target); /* includes EXDEV: cross-device move is a non-goal */
    return -1;
}
int main(int argc, char **argv) {
    init_program(argv[0]);
    bool no_clobber = false, verbose = false, no_target_dir = false;
    const struct option options[] = {{"help",0,NULL,1000},{0,0,0,0}};
    int ch;
    while ((ch = getopt_long(argc, argv, "fnvT", options, NULL)) != -1) {
        switch (ch) {
        case 'f': no_clobber = false; break;
        case 'n': no_clobber = true; break;
        case 'v': verbose = true; break;
        case 'T': no_target_dir = true; break;
        case 1000: usage(); return 0;
        default: usage(); return 1;
        }
    }
    if (argc - optind < 2) { usage(); return 1; }
    char *destination = clean_path(argv[argc - 1]);
    struct stat st;
    bool target_dir = !no_target_dir && stat(destination, &st) == 0 && S_ISDIR(st.st_mode);
    if (argc - optind > 2 && !target_dir) {
        fprintf(stderr, "%s: destination must be a directory\n", program); free(destination); return 1;
    }
    int failed = 0;
    for (int i = optind; i < argc - 1; ++i) {
        char *source = clean_path(argv[i]);
        char *target = target_dir ? join_path(destination, base_name(source)) : xstrdup(destination);
        if (move_one(source, target, no_clobber, verbose) < 0) failed = 1;
        free(source); free(target);
    }
    free(destination);
    return failed;
}
