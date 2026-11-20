#include "common.h"
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

const char *program = "fileutils";

void init_program(const char *argv0) { program = base_name(argv0); }
const char *base_name(const char *path) {
    const char *slash = strrchr(path, '/');
    return slash ? slash + 1 : path;
}
void error_path(const char *operation, const char *path) {
    int saved = errno;
    fprintf(stderr, "%s: %s '%s': %s\n", program, operation, path, strerror(saved));
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
int remove_tree(const char *path, bool recursive, bool dirs, bool force,
                bool interactive, bool verbose) {
    struct stat st;
    if (lstat(path, &st) < 0) {
        if (force && errno == ENOENT) return 0;
        error_path("cannot access", path); return -1;
    }
    if (S_ISDIR(st.st_mode)) {
        if (!recursive && !dirs) {
            errno = EISDIR; error_path("cannot remove", path); return -1;
        }
        if (recursive) {
            if (interactive && !confirm("descend into directory", path)) return 1;
            DIR *dir = opendir(path);
            if (!dir) { error_path("cannot open directory", path); return -1; }
            int failed = 0, skipped = 0;
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
                int rc = remove_tree(child, true, dirs, force, interactive, verbose);
                if (rc < 0) failed = 1;
                if (rc > 0) skipped = 1;
                free(child);
            }
            if (closedir(dir) < 0) { error_path("cannot close directory", path); failed = 1; }
            if (failed) return -1;
            if (skipped) return 1;
        }
    }
    if (interactive && !confirm("remove", path)) return 1;
    int rc = S_ISDIR(st.st_mode) ? rmdir(path) : unlink(path);
    if (rc < 0) { error_path("cannot remove", path); return -1; }
    if (verbose) printf("removed '%s'\n", path);
    return 0;
}

/* Cross-filesystem mv fallback. Never traverse symbolic links. */
int copy_tree(const char *source, const char *target) {
    struct stat st;
    if (lstat(source, &st) < 0) { error_path("cannot stat", source); return -1; }
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
        if (rc < 0) { error_path("cannot create link", target); return -1; }
        struct timespec times[2] = { st.st_atim, st.st_mtim };
        if (utimensat(AT_FDCWD, target, times, AT_SYMLINK_NOFOLLOW) < 0) {
            error_path("cannot set timestamps", target); return -1;
        }
        return 0;
    }
    if (S_ISDIR(st.st_mode)) {
        if (mkdir(target, 0700) < 0) { error_path("cannot create directory", target); return -1; }
        DIR *dir = opendir(source);
        if (!dir) { error_path("cannot open directory", source); return -1; }
        int failed = 0;
        for (;;) {
            errno = 0;
            struct dirent *entry = readdir(dir);
            if (!entry) {
                if (errno) { error_path("cannot read directory", source); failed = 1; }
                break;
            }
            if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
            char *src = join_path(source, entry->d_name);
            char *dst = join_path(target, entry->d_name);
            if (copy_tree(src, dst) < 0) failed = 1;
            free(src); free(dst);
            if (failed) break;
        }
        if (closedir(dir) < 0) { error_path("cannot close directory", source); failed = 1; }
        if (failed) return -1;
    } else if (S_ISREG(st.st_mode)) {
        int in = open(source, O_RDONLY | O_NOFOLLOW);
        if (in < 0) { error_path("cannot open", source); return -1; }
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
        errno = ENOTSUP; error_path("cannot copy special file", source); return -1;
    }
    if (chmod(target, st.st_mode & 07777) < 0) { error_path("cannot set mode", target); return -1; }
    struct timespec times[2] = { st.st_atim, st.st_mtim };
    if (utimensat(AT_FDCWD, target, times, 0) < 0) {
        error_path("cannot set timestamps", target); return -1;
    }
    return 0;
}
