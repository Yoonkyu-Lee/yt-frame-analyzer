#include "common.h"
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const char *program = "fileutils";

void init_program(const char *argv0) { program = base_name(argv0); }
/* C2: index of the first operand; a leading "--" only ends the (empty) option list. */
int first_operand(int argc, char **argv) { return 1 + (argc > 1 && !strcmp(argv[1], "--")); }
const char *base_name(const char *path) {
    const char *slash = strrchr(path, '/');
    return slash ? slash + 1 : path;
}
void error_path(const char *operation, const char *path) {
    fprintf(stderr, "%s: %s '%s': %s\n", program, operation, path, strerror(errno));
}
void *xmalloc(size_t size) {
    void *p = malloc(size);
    if (!p) { perror(program); exit(1); }
    return p;
}
char *xstrdup(const char *s) {
    char *p = xmalloc(strlen(s) + 1);
    strcpy(p, s);
    return p;
}
char *join_path(const char *left, const char *right) {
    size_t n = strlen(left), m = strlen(right);
    char *p = xmalloc(n + m + 2);
    memcpy(p, left, n);
    if (n && left[n - 1] != '/') p[n++] = '/';
    memcpy(p + n, right, m + 1);
    return p;
}
char *clean_path(const char *path) {
    char *p = xstrdup(path);
    size_t n = strlen(p);
    while (n > 1 && p[n - 1] == '/') p[--n] = '\0';
    return p;
}
/* I5, I6: ARG without trailing slashes, but "file/" still fails (NULL, errno ENOTDIR) as the kernel would. */
char *operand_path(const char *arg) {
    char *p = clean_path(arg);
    struct stat st;
    if (strlen(p) < strlen(arg) && stat(arg, &st) < 0 && errno == ENOTDIR) { free(p); return NULL; }
    return p;
}
