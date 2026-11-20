#include "common.h"
#include <dirent.h>
#include <errno.h>
#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static void usage(void) {
    printf("Usage: %s [-frdRv] FILE...\n"
           "  -r, -R recursive  -d empty directories  -f ignore missing\n"
           "  -v report removals\n", program);
}

/* lstat is deliberate: removing a directory symlink removes only the link. */
static int remove_tree(const char *path, bool recursive, bool dirs, bool force, bool verbose) {
    struct stat st;
    if (lstat(path, &st) < 0) {
        if (force && errno == ENOENT) return 0;
        error_path("cannot access", path); return -1;
    }
    if (S_ISDIR(st.st_mode)) {
        if (!recursive && !dirs) {
            errno = EISDIR; error_path("cannot remove", path); return -1;
        }
        if (recursive) {
            DIR *dir = opendir(path);
            if (!dir) { error_path("cannot open directory", path); return -1; }
            int failed = 0;
            struct dirent *entry;
            for (;;) {
                errno = 0;
                entry = readdir(dir);
                if (!entry) {
                    if (errno) { error_path("cannot read directory", path); failed = 1; }
                    break;
                }
                if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
                char *child = join_path(path, entry->d_name);
                if (remove_tree(child, true, dirs, force, verbose) < 0) failed = 1;
                free(child);
            }
            if (closedir(dir) < 0) { error_path("cannot close directory", path); failed = 1; }
            if (failed) return -1;
        }
    }
    int rc = S_ISDIR(st.st_mode) ? rmdir(path) : unlink(path);
    if (rc < 0) { error_path("cannot remove", path); return -1; }
    if (verbose) printf("removed '%s'\n", path);
    return 0;
}

int main(int argc, char **argv) {
    init_program(argv[0]);
    bool recursive = false, dirs = false, force = false, verbose = false;
    const struct option options[] = {{"help",0,NULL,1000},{0,0,0,0}};
    int ch;
    while ((ch = getopt_long(argc, argv, "frdRv", options, NULL)) != -1) {
        switch (ch) {
        case 'f': force = true; break;
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
        } else if (remove_tree(path, recursive, dirs, force, verbose) < 0) failed = 1;
        free(path);
    }
    return failed;
}
