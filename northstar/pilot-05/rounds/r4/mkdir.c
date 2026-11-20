#include "common.h"
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

/* I3: create one directory; with -p an existing directory is accepted (a file is not). */
static int create(const char *path, mode_t mode, bool parents) {
    if (mkdir(path, mode) == 0) return 0;
    if (parents && errno == EEXIST) {
        struct stat st;
        if (stat(path, &st) == 0 && S_ISDIR(st.st_mode)) return 0;
        errno = EEXIST;
    }
    error_path("cannot create directory", path);
    return -1;
}

/* I3, C1, C2: [-p] DIR...; -p creates each missing parent first. */
int main(int argc, char **argv) {
    init_program(argv[0]);
    bool parents = false;
    mode_t mask = umask(0); umask(mask);
    int ch;
    while ((ch = getopt(argc, argv, "p")) != -1) {
        if (ch != 'p') { printf("Usage: %s [-p] DIRECTORY...\n", program); return 1; }
        parents = true;
    }
    if (optind == argc) { printf("Usage: %s [-p] DIRECTORY...\n", program); return 1; }
    int failed = 0;
    for (int i = optind; i < argc; ++i) {
        char *path = clean_path(argv[i]);
        bool ok = true;
        /* -p: create "a", then "a/b", ... ("a//b" just meets "a" twice, which -p accepts) */
        for (char *p = path; parents && ok && *p && (p = strchr(p + 1, '/'));) {
            *p = '\0';
            /* Parent directories must remain traversable even under a restrictive umask. */
            mode_t previous = umask(0);
            ok = create(path, (0777 & ~mask) | S_IWUSR | S_IXUSR, true) == 0;
            umask(previous);
            *p = '/';
        }
        if (!ok || create(path, 0777 & ~mask, parents) < 0) failed = 1;
        free(path);
    }
    return failed;
}
