#include "common.h"
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

static const char help[] =
    "[-pv] [-m OCTAL_MODE] DIRECTORY...\n"
    "  -p create parents, accept existing directories  -v report creations\n";
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
    return error_path("cannot create directory", path);
}
int main(int argc, char **argv) {
    program = base_name(argv[0]);
    bool parents = false, verbose = false, explicit_mode = false;
    mode_t mask = umask(0);
    mode_t mode = 0777 & ~mask;
    int ch;
    while ((ch = next_option(argc, argv, "pvm:", help)) != -1) {
        switch (ch) {
        case 'p': parents = true; break;
        case 'v': verbose = true; break;
        case 'm':
            explicit_mode = true;
            if (parse_mode(optarg, &mode) < 0) {
                fprintf(stderr, "%s: invalid mode '%s'\n", program, optarg); return 1;
            }
            break;
        }
    }
    if (optind == argc) usage(help, 1);
    int failed = 0;
    for (int i = optind; i < argc; ++i) {
        char *path = clean_path(argv[i]);
        bool ok = true;
        if (parents) {
            for (char *p = path + (*path == '/'); *p; ++p) {
                if (*p != '/' || p[-1] == '/') continue;
                *p = '\0';
                /* Parent directories must remain traversable even under a restrictive umask. */
                int rc = create(path, (0777 & ~mask) | S_IWUSR | S_IXUSR, true, verbose);
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
