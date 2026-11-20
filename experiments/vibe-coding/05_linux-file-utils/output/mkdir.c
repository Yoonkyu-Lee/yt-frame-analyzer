#include "common.h"
#include <errno.h>
#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static void usage(void) {
    printf("Usage: %s [-pv] [-m OCTAL_MODE] DIRECTORY...\n"
           "  -p create parents, accept existing directories  -v report creations\n", program);
}
static int create(const char *path, mode_t mode, bool parents, bool verbose) {
    if (mkdir(path, mode) == 0) {
        if (verbose) printf("created directory '%s'\n", path);
        return 1;
    }
    if (parents && errno == EEXIST) {
        struct stat st;
        if (stat(path, &st) == 0 && S_ISDIR(st.st_mode)) return 0;
        errno = EEXIST;
    }
    error_path("cannot create directory", path);
    return -1;
}
int main(int argc, char **argv) {
    init_program(argv[0]);
    bool parents = false, verbose = false, explicit_mode = false;
    mode_t mask = umask(0); umask(mask);
    mode_t mode = 0777 & ~mask;
    const struct option options[] = {{"help",0,NULL,1000},{0,0,0,0}};
    int ch;
    while ((ch = getopt_long(argc, argv, "pvm:", options, NULL)) != -1) {
        switch (ch) {
        case 'p': parents = true; break;
        case 'v': verbose = true; break;
        case 'm':
            explicit_mode = true;
            if (parse_mode(optarg, &mode) < 0) {
                fprintf(stderr, "%s: invalid mode '%s'\n", program, optarg); return 1;
            }
            break;
        case 1000: usage(); return 0;
        default: usage(); return 1;
        }
    }
    if (optind == argc) { usage(); return 1; }
    int failed = 0;
    for (int i = optind; i < argc; ++i) {
        char *path = clean_path(argv[i]);
        bool ok = true;
        if (parents) {
            for (char *p = path + (*path == '/' ? 1 : 0); *p; ++p) {
                if (*p != '/' || p == path || p[-1] == '/') continue;
                *p = '\0';
                /* Parent directories must remain traversable even under a restrictive umask. */
                mode_t parent_mode = (0777 & ~mask) | S_IWUSR | S_IXUSR;
                mode_t previous = umask(0);
                int rc = create(path, parent_mode, true, verbose);
                umask(previous);
                *p = '/';
                if (rc < 0) { ok = false; break; }
            }
        }
        if (ok) {
            int rc = create(path, mode, parents, verbose);
            if (rc < 0) failed = 1;
            else if (rc > 0 && explicit_mode && chmod(path, mode) < 0) {
                error_path("cannot set mode", path); failed = 1;
            }
        } else failed = 1;
        free(path);
    }
    return failed;
}
