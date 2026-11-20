#include "common.h"
#include <dirent.h>
#include <errno.h>
#include <fnmatch.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char *name_glob; /* -name GLOB, NULL = any name */
static char type;             /* -type f|d, 0 = any type */
static int failed;

/* I6: does PATH pass every test given on the command line? */
static bool matches(const char *path, const struct stat *st) {
    if (name_glob && fnmatch(name_glob, base_name(path), 0) != 0) return false;
    if (type == 'f') return S_ISREG(st->st_mode);
    if (type == 'd') return S_ISDIR(st->st_mode);
    return true;
}

/* I6: print PATH if it matches, then walk into it; lstat so links are never followed. */
static void walk(const char *path) {
    struct stat st;
    if (lstat(path, &st) < 0) { error_path("cannot stat", path); failed = 1; return; }
    if (matches(path, &st)) puts(path);
    if (!S_ISDIR(st.st_mode)) return;
    DIR *dir = opendir(path);
    if (!dir) { error_path("cannot open directory", path); failed = 1; return; }
    for (;;) {
        errno = 0;
        struct dirent *entry = readdir(dir);
        if (!entry) {
            if (errno) { error_path("cannot read directory", path); failed = 1; }
            break;
        }
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        char *child = join_path(path, entry->d_name);
        walk(child);
        free(child);
    }
    closedir(dir);
}

/* I6, C1: [PATH...] (default ".") then -name GLOB and/or -type f|d. */
int main(int argc, char **argv) {
    init_program(argv[0]);
    int end = 1;
    while (end < argc && argv[end][0] != '-') ++end;
    for (int i = end; i < argc; i += 2) {
        const char *value = i + 1 < argc ? argv[i + 1] : NULL;
        if (value && !strcmp(argv[i], "-name")) name_glob = value;
        else if (value && !strcmp(argv[i], "-type") && (!strcmp(value, "f") || !strcmp(value, "d"))) type = *value;
        else { fprintf(stderr, "%s: invalid expression at '%s'\n", program, argv[i]); return 1; }
    }
    if (end == 1) walk(".");
    for (int i = 1; i < end; ++i) {
        char *path = clean_path(argv[i]);
        walk(path);
        free(path);
    }
    if (fflush(stdout) == EOF) { error_path("cannot write", "standard output"); failed = 1; }
    return failed;
}
