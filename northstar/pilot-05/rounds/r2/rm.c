#include "common.h"
#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

/* I5: remove PATH; a directory only when recursive; lstat so links are removed, never followed. */
static int remove_tree(const char *path, bool recursive, bool force) {
    struct stat st;
    if (lstat(path, &st) < 0) {
        if (force && errno == ENOENT) return 0;
        error_path("cannot access", path); return -1;
    }
    if (S_ISDIR(st.st_mode)) {
        if (!recursive) { errno = EISDIR; error_path("cannot remove", path); return -1; }
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
            if (remove_tree(child, true, force) < 0) failed = 1;
            free(child);
        }
        if (closedir(dir) < 0) { error_path("cannot close directory", path); failed = 1; }
        if (failed) return -1;
    }
    int rc = S_ISDIR(st.st_mode) ? rmdir(path) : unlink(path);
    if (rc < 0) { error_path("cannot remove", path); return -1; }
    return 0;
}

/* I5, C1, C2: [-r] [-f] PATH...; refuse ".", ".." and "/". */
int main(int argc, char **argv) {
    init_program(argv[0]);
    bool recursive = false, force = false;
    int ch;
    while ((ch = getopt(argc, argv, "rf")) != -1) {
        if (ch == 'r') recursive = true;
        else if (ch == 'f') force = true;
        else { printf("Usage: %s [-rf] FILE...\n", program); return 1; }
    }
    if (optind == argc) { if (force) return 0; printf("Usage: %s [-rf] FILE...\n", program); return 1; }
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
        } else if (remove_tree(path, recursive, force) < 0) failed = 1;
        free(path);
    }
    return failed;
}
