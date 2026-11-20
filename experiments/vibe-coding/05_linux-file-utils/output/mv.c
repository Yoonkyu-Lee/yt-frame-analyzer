#include "common.h"
#include <errno.h>
#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static void usage(void) {
    printf("Usage: %s [-finvT] SOURCE... DESTINATION\n"
           "  -f overwrite  -i prompt before overwrite  -n no overwrite\n"
           "  -v report moves  -T treat destination as a file\n", program);
}
static int move_one(const char *source, const char *target, bool interactive, bool no_clobber, bool verbose) {
    struct stat src, dst;
    if (lstat(source, &src) < 0) { error_path("cannot stat", source); return -1; }
    int exists = lstat(target, &dst) == 0;
    if (!exists && errno != ENOENT) { error_path("cannot stat", target); return -1; }
    if (exists) {
        if (src.st_dev == dst.st_dev && src.st_ino == dst.st_ino) {
            fprintf(stderr, "%s: '%s' and '%s' are the same file\n", program, source, target); return -1;
        }
        if (no_clobber || (interactive && !confirm("overwrite", target))) return 0;
    }
    if (rename(source, target) == 0) {
        if (verbose) printf("'%s' -> '%s'\n", source, target);
        return 0;
    }
    if (errno != EXDEV) { error_path("cannot move to", target); return -1; }
    if (exists) {
        if (S_ISDIR(src.st_mode) != S_ISDIR(dst.st_mode)) {
            errno = S_ISDIR(dst.st_mode) ? EISDIR : ENOTDIR;
            error_path("cannot replace", target); return -1;
        }
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
    if (rc == 0) rc = remove_tree(source, true, true, false, false, false);
    if (remove_tree(temporary, true, true, false, false, false) < 0) rc = -1;
    if (rc == 0 && verbose) printf("'%s' -> '%s'\n", source, target);
    free(staged); free(temporary);
    return rc;
}
int main(int argc, char **argv) {
    init_program(argv[0]);
    bool interactive = false, no_clobber = false, verbose = false, no_target_dir = false;
    const struct option options[] = {{"help",0,NULL,1000},{0,0,0,0}};
    int ch;
    while ((ch = getopt_long(argc, argv, "finvT", options, NULL)) != -1) {
        switch (ch) {
        case 'f': interactive = no_clobber = false; break;
        case 'i': interactive = true; no_clobber = false; break;
        case 'n': no_clobber = true; interactive = false; break;
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
        if (move_one(source, target, interactive, no_clobber, verbose) < 0) failed = 1;
        free(source); free(target);
    }
    free(destination);
    return failed;
}
