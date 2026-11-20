#include "common.h"
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

const char *program = "fileutils";

_Noreturn void usage(const char *text, int status) {
    printf("Usage: %s %s", program, text);
    exit(status);
}
int next_option(int argc, char **argv, const char *options, const char *help) {
    enum { HELP = 256 };
    static const struct option long_options[] = {{"help", 0, NULL, HELP}, {0}};
    int ch = getopt_long(argc, argv, options, long_options, NULL);
    if (ch == HELP || ch == '?') usage(help, ch == '?');
    return ch;
}
const char *base_name(const char *path) {
    const char *slash = strrchr(path, '/');
    return slash ? slash + 1 : path;
}
int error_path(const char *operation, const char *path) {
    int saved = errno;
    fprintf(stderr, "%s: %s '%s': %s\n", program, operation, path, strerror(saved));
    return -1;
}
struct dirent *read_entry(DIR *dir, const char *path, int *failed) {
    struct dirent *entry;
    do {
        errno = 0;
        entry = readdir(dir);
    } while (entry && (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")));
    if (!entry && errno) {
        error_path("cannot read directory", path);
        *failed = 1;
    }
    return entry;
}
int close_dir(DIR *dir, const char *path) {
    return closedir(dir) < 0 ? error_path("cannot close directory", path) : 0;
}
void *xmalloc(size_t size) {
    void *p = malloc(size ? size : 1);
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
bool confirm(const char *operation, const char *path) {
    fprintf(stderr, "%s: %s '%s'? ", program, operation, path);
    fflush(stderr);
    int c = getchar(), first = c;
    while (c != '\n' && c != EOF) c = getchar();
    return first == 'y' || first == 'Y';
}
int parse_mode(const char *text, mode_t *mode) {
    char *end;
    errno = 0;
    unsigned long n = strtoul(text, &end, 8);
    if (!*text || *text == '-' || *end || errno || n > 07777) return -1;
    *mode = (mode_t)n;
    return 0;
}

/* lstat is deliberate: removing a directory symlink removes only the link. */
int remove_tree(const char *path, unsigned flags) {
    struct stat st;
    if (lstat(path, &st) < 0) {
        if ((flags & FORCE) && errno == ENOENT) return 0;
        return error_path("cannot access", path);
    }
    if (S_ISDIR(st.st_mode)) {
        if (!(flags & (RECURSIVE | DIRECTORIES))) {
            errno = EISDIR; return error_path("cannot remove", path);
        }
        if (flags & RECURSIVE) {
            if ((flags & INTERACTIVE) && !confirm("descend into directory", path)) return 1;
            DIR *dir = opendir(path);
            if (!dir) return error_path("cannot open directory", path);
            int failed = 0, skipped = 0;
            struct dirent *entry;
            while ((entry = read_entry(dir, path, &failed))) {
                char *child = join_path(path, entry->d_name);
                int rc = remove_tree(child, flags);
                if (rc < 0) failed = 1;
                if (rc > 0) skipped = 1;
                free(child);
            }
            if (close_dir(dir, path) < 0) failed = 1;
            if (failed) return -1;
            if (skipped) return 1;
        }
    }
    if ((flags & INTERACTIVE) && !confirm("remove", path)) return 1;
    int rc = S_ISDIR(st.st_mode) ? rmdir(path) : unlink(path);
    if (rc < 0) return error_path("cannot remove", path);
    if (flags & VERBOSE) printf("removed '%s'\n", path);
    return 0;
}

/* Cross-filesystem mv fallback. Never traverse symbolic links. */
int copy_tree(const char *source, const char *target) {
    struct stat st;
    if (lstat(source, &st) < 0) return error_path("cannot stat", source);
    if (S_ISLNK(st.st_mode)) {
        size_t size = st.st_size > 0 ? (size_t)st.st_size + 1 : 256;
        char *link;
        for (;;) {
            link = xmalloc(size + 1);
            ssize_t n = readlink(source, link, size);
            if (n < 0) { error_path("cannot read link", source); free(link); return -1; }
            if ((size_t)n < size) { link[n] = '\0'; break; }
            free(link); size *= 2;
        }
        int rc = symlink(link, target);
        free(link);
        if (rc < 0) return error_path("cannot create link", target);
    } else if (S_ISDIR(st.st_mode)) {
        if (mkdir(target, 0700) < 0) return error_path("cannot create directory", target);
        DIR *dir = opendir(source);
        if (!dir) return error_path("cannot open directory", source);
        int failed = 0;
        struct dirent *entry;
        while ((entry = read_entry(dir, source, &failed))) {
            char *src = join_path(source, entry->d_name);
            char *dst = join_path(target, entry->d_name);
            if (copy_tree(src, dst) < 0) failed = 1;
            free(src); free(dst);
            if (failed) break;
        }
        if (close_dir(dir, source) < 0) failed = 1;
        if (failed) return -1;
    } else if (S_ISREG(st.st_mode)) {
        int in = open(source, O_RDONLY | O_NOFOLLOW);
        if (in < 0) return error_path("cannot open", source);
        int out = open(target, O_WRONLY | O_CREAT | O_EXCL, 0600);
        if (out < 0) { error_path("cannot create", target); close(in); return -1; }
        char buffer[65536];
        int failed = 0;
        for (;;) {
            ssize_t n = read(in, buffer, sizeof buffer);
            if (n < 0 && errno == EINTR) continue;
            if (n < 0) { error_path("cannot read", source); failed = 1; break; }
            if (!n) break;
            for (ssize_t offset = 0; offset < n;) {
                ssize_t written = write(out, buffer + offset, (size_t)(n - offset));
                if (written < 0 && errno == EINTR) continue;
                if (written <= 0) { error_path("cannot write", target); failed = 1; break; }
                offset += written;
            }
            if (failed) break;
        }
        if (close(in) < 0) { error_path("cannot close", source); failed = 1; }
        if (close(out) < 0) { error_path("cannot close", target); failed = 1; }
        if (failed) return -1;
    } else {
        errno = ENOTSUP; return error_path("cannot copy special file", source);
    }
    if (!S_ISLNK(st.st_mode) && chmod(target, st.st_mode & 07777) < 0)
        return error_path("cannot set mode", target);
    struct timespec times[2] = { st.st_atim, st.st_mtim };
    return utimensat(AT_FDCWD, target, times, AT_SYMLINK_NOFOLLOW) < 0
        ? error_path("cannot set timestamps", target) : 0;
}
