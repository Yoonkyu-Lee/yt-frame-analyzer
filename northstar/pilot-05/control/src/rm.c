#include "common.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char help[] =
    "[-firdRv] FILE...\n"
    "  -r, -R recursive  -d empty directories  -f ignore missing, never prompt\n"
    "  -i prompt before removal  -v report removals\n";
int main(int argc, char **argv) {
    program = base_name(argv[0]);
    unsigned flags = 0;
    int ch;
    while ((ch = next_option(argc, argv, "firdRv", help)) != -1) {
        switch (ch) {
        case 'f': flags = (flags | FORCE) & ~INTERACTIVE; break;
        case 'i': flags = (flags | INTERACTIVE) & ~FORCE; break;
        case 'r': case 'R': flags |= RECURSIVE; break;
        case 'd': flags |= DIRECTORIES; break;
        case 'v': flags |= VERBOSE; break;
        }
    }
    if (optind == argc) { if (flags & FORCE) return 0; usage(help, 1); }
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
        } else if (remove_tree(path, flags) < 0) failed = 1;
        free(path);
    }
    return failed;
}
