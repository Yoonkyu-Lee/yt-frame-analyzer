#include "common.h"
#include <dirent.h>
#include <errno.h>
#include <fnmatch.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static char **tests;  /* "-name GLOB" / "-type f|d" pairs from the command line */
static int ntests;
static int failed;

/* I6: does PATH pass every test given on the command line? */
static bool matches(const char *path, const struct stat *st) {
    for (int i = 0; i < ntests; i += 2) {
        const char *value = tests[i + 1];
        bool ok = tests[i][1] == 'n' ? fnmatch(value, base_name(path), 0) == 0
                : *value == 'f'      ? S_ISREG(st->st_mode) : S_ISDIR(st->st_mode);
        if (!ok) return false;
    }
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
    struct dirent *entry;
    while ((errno = 0, entry = readdir(dir))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        char *child = join_path(path, entry->d_name);
        walk(child);
        free(child);
    }
    if (errno) { error_path("cannot read directory", path); failed = 1; }
    closedir(dir);
}

/* I6, C1: [PATH...] (default ".") then -name GLOB and/or -type f|d, each any number of times. */
int main(int argc, char **argv) {
    init_program(argv[0]);
    int first = first_operand(argc, argv), end = first;
    while (end < argc && argv[end][0] != '-') ++end;
    for (int i = end; i < argc; i += 2) {
        const char *value = i + 1 < argc ? argv[i + 1] : NULL;
        if (value && !strcmp(argv[i], "-name")) continue;
        if (value && !strcmp(argv[i], "-type") && (!strcmp(value, "f") || !strcmp(value, "d"))) continue;
        fprintf(stderr, "%s: invalid expression at '%s'\n", program, argv[i]); return 1;
    }
    tests = argv + end; ntests = argc - end;
    if (end == first) walk(".");
    for (int i = first; i < end; ++i) {
        char *path = operand_path(argv[i]);
        if (!path) { error_path("cannot stat", argv[i]); failed = 1; continue; }
        walk(path);
        free(path);
    }
    if (fflush(stdout) == EOF || ferror(stdout)) { error_path("cannot write", "standard output"); failed = 1; }
    return failed;
}
