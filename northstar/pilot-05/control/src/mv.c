#include "common.h"
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char help[] =
    "[-finvT] SOURCE... DESTINATION\n"
    "  -f overwrite  -i prompt before overwrite  -n no overwrite\n"
    "  -v report moves  -T treat destination as a file\n";
static int move_one(const char *source, const char *target, int overwrite, bool verbose) {
    struct stat src, dst;
    if (lstat(source, &src) < 0) return error_path("cannot stat", source);
    int exists = lstat(target, &dst) == 0;
    if (!exists && errno != ENOENT) return error_path("cannot stat", target);
    if (exists) {
        if (src.st_dev == dst.st_dev && src.st_ino == dst.st_ino) {
            fprintf(stderr, "%s: '%s' and '%s' are the same file\n", program, source, target); return -1;
        }
        if (overwrite == 'n' || (overwrite == 'i' && !confirm("overwrite", target))) return 0;
    }
    if (rename(source, target) == 0) {
        if (verbose) printf("'%s' -> '%s'\n", source, target);
        return 0;
    }
    if (errno != EXDEV) return error_path("cannot move to", target);
    if (exists && S_ISDIR(src.st_mode) != S_ISDIR(dst.st_mode)) {
        errno = S_ISDIR(dst.st_mode) ? EISDIR : ENOTDIR;
        return error_path("cannot replace", target);
    }
    /* Stage beside the target, so a failed copy cannot destroy an existing target. */
    char *parent = xstrdup(target);
    char *slash = strrchr(parent, '/');
    if (slash) { if (slash == parent) slash[1] = '\0'; else *slash = '\0'; }
    else { free(parent); parent = xstrdup("."); }
    char *temporary = join_path(parent, ".fileutils-mv-XXXXXX");
    free(parent);
    if (!mkdtemp(temporary)) { error_path("cannot create staging directory", temporary); free(temporary); return -1; }
    char *staged = join_path(temporary, "content");
    int rc = copy_tree(source, staged);
    if (rc == 0 && rename(staged, target) < 0) { error_path("cannot replace", target); rc = -1; }
    if (rc == 0) rc = remove_tree(source, RECURSIVE);
    if (remove_tree(temporary, RECURSIVE) < 0) rc = -1;
    if (rc == 0 && verbose) printf("'%s' -> '%s'\n", source, target);
    free(staged); free(temporary);
    return rc;
}
int main(int argc, char **argv) {
    program = base_name(argv[0]);
    bool verbose = false, no_target_dir = false;
    int overwrite = 'f';
    int ch;
    while ((ch = next_option(argc, argv, "finvT", help)) != -1) {
        switch (ch) {
        case 'f': case 'i': case 'n': overwrite = ch; break;
        case 'v': verbose = true; break;
        case 'T': no_target_dir = true; break;
        }
    }
    if (argc - optind < 2) usage(help, 1);
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
        if (move_one(source, target, overwrite, verbose) < 0) failed = 1;
        free(source); free(target);
    }
    free(destination);
    return failed;
}
