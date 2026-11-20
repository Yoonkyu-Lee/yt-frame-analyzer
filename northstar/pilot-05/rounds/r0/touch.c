#include "common.h"
#include <errno.h>
#include <fcntl.h>
#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

static void usage(void) {
    printf("Usage: %s [-acmh] [-r REFERENCE | -t YYYYMMDDhhmm[.ss]] FILE...\n"
           "Update access and modification times; create absent files unless -c.\n"
           "  -a access only  -m modification only  -h affect symlink itself\n", program);
}
static int timestamp(const char *s, struct timespec *out) {
    size_t length = strlen(s);
    if (length != 12 && length != 15) return -1;
    for (size_t i = 0; i < length; ++i) {
        if (i == 12) { if (s[i] != '.') return -1; }
        else if (s[i] < '0' || s[i] > '9') return -1;
    }
    struct tm tm = {0};
    char *end = strptime(s, "%Y%m%d%H%M", &tm);
    if (!end) return -1;
    if (*end == '.') {
        char *next;
        long sec = strtol(end + 1, &next, 10);
        if (next != end + 3 || *next || sec < 0 || sec > 59) return -1;
        tm.tm_sec = (int)sec;
    } else if (*end) return -1;
    struct tm requested = tm;
    tm.tm_isdst = -1;
    time_t t = mktime(&tm);
    if (tm.tm_year != requested.tm_year || tm.tm_mon != requested.tm_mon ||
        tm.tm_mday != requested.tm_mday || tm.tm_hour != requested.tm_hour ||
        tm.tm_min != requested.tm_min) return -1;
    out->tv_sec = t; out->tv_nsec = 0;
    return 0;
}
int main(int argc, char **argv) {
    init_program(argv[0]);
    bool access_only = false, modify_only = false, no_create = false, no_follow = false;
    const char *reference = NULL, *stamp = NULL;
    const struct option options[] = {{"help", 0, NULL, 1000}, {0,0,0,0}};
    int ch;
    while ((ch = getopt_long(argc, argv, "acmhr:t:", options, NULL)) != -1) {
        switch (ch) {
        case 'a': access_only = true; break;
        case 'm': modify_only = true; break;
        case 'c': no_create = true; break;
        case 'h': no_follow = true; break;
        case 'r': reference = optarg; break;
        case 't': stamp = optarg; break;
        case 1000: usage(); return 0;
        default: usage(); return 1;
        }
    }
    if (optind == argc || (reference && stamp)) { usage(); return 1; }
    struct timespec times[2] = {{0, UTIME_NOW}, {0, UTIME_NOW}};
    if (reference) {
        struct stat st;
        if ((no_follow ? lstat(reference, &st) : stat(reference, &st)) < 0) {
            error_path("cannot stat", reference); return 1;
        }
        times[0] = st.st_atim; times[1] = st.st_mtim;
    }
    if (stamp) {
        if (timestamp(stamp, &times[0]) < 0) {
            fprintf(stderr, "%s: invalid timestamp '%s'\n", program, stamp); return 1;
        }
        times[1] = times[0];
    }
    if (access_only && !modify_only) times[1].tv_nsec = UTIME_OMIT;
    if (modify_only && !access_only) times[0].tv_nsec = UTIME_OMIT;
    int failed = 0;
    for (int i = optind; i < argc; ++i) {
        int flags = no_follow ? AT_SYMLINK_NOFOLLOW : 0;
        if (utimensat(AT_FDCWD, argv[i], times, flags) == 0) continue;
        if (errno == ENOENT && no_create) continue;
        if (errno == ENOENT && !no_follow) {
            int fd = open(argv[i], O_WRONLY | O_CREAT | O_NONBLOCK, 0666);
            if (fd < 0) { error_path("cannot create", argv[i]); failed = 1; continue; }
            int rc = futimens(fd, times);
            if (rc < 0) { error_path("cannot set timestamps", argv[i]); failed = 1; }
            if (close(fd) < 0) { error_path("cannot close", argv[i]); failed = 1; }
        } else { error_path("cannot set timestamps", argv[i]); failed = 1; }
    }
    return failed;
}
