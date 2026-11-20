#include "common.h"
#include <errno.h>
#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void usage(void) {
    printf("Usage: %s [-firdRv] FILE...\n"
           "  -r, -R recursive  -d empty directories  -f ignore missing, never prompt\n"
           "  -i prompt before removal  -v report removals\n", program);
}
int main(int argc, char **argv) {
    init_program(argv[0]);
    bool recursive = false, dirs = false, force = false, interactive = false, verbose = false;
    const struct option options[] = {{"help",0,NULL,1000},{0,0,0,0}};
    int ch;
    while ((ch = getopt_long(argc, argv, "firdRv", options, NULL)) != -1) {
        switch (ch) {
        case 'f': force = true; interactive = false; break;
        case 'i': interactive = true; force = false; break;
        case 'r': case 'R': recursive = true; break;
        case 'd': dirs = true; break;
        case 'v': verbose = true; break;
        case 1000: usage(); return 0;
        default: usage(); return 1;
        }
    }
    if (optind == argc) { if (force) return 0; usage(); return 1; }
    int failed = 0;
    struct stat root;
    if (stat("/", &root) < 0) { error_path("cannot stat", "/"); return 1; }
    for (int i = optind; i < argc; ++i) {
        char *path = clean_path(argv[i]);
        const char *base = base_name(path);
        struct stat st;
        bool dot = !strcmp(base, ".") || !strcmp(base, "..");
        bool is_root = lstat(path, &st) == 0 && S_ISDIR(st.st_mode) &&
                       st.st_dev == root.st_dev && st.st_ino == root.st_ino;
        if (dot || is_root) {
            fprintf(stderr, "%s: refusing to remove '%s'\n", program, argv[i]); failed = 1;
        } else if (remove_tree(path, recursive, dirs, force, interactive, verbose) < 0) failed = 1;
        free(path);
    }
    return failed;
}
