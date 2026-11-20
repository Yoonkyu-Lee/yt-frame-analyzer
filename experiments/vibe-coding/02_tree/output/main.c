#define _POSIX_C_SOURCE 200809L

#include <dirent.h>
#include <errno.h>
#include <fnmatch.h>
#include <getopt.h>
#include <inttypes.h>
#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

typedef struct {
    bool all, dirs_only, follow, full_path, classify, ascii, dirs_first;
    bool report;
    unsigned long max_depth;
    const char *ignore;
} Options;

typedef struct {
    char *name;
    struct stat info;
    bool directory, symlink;
    int error;
} Entry;

typedef struct Ancestor {
    dev_t device;
    ino_t inode;
    const struct Ancestor *parent;
} Ancestor;

static Options opt = {.report = true, .max_depth = ULONG_MAX};
static uintmax_t directories, files;
static int failed;

static void *allocate(size_t size)
{
    void *p = malloc(size);
    if (!p) {
        fputs("ctree: out of memory\n", stderr);
        exit(EXIT_FAILURE);
    }
    return p;
}

static char *copy_string(const char *s)
{
    size_t n = strlen(s) + 1;
    char *p = allocate(n);
    memcpy(p, s, n);
    return p;
}

static char *join_path(const char *parent, const char *name)
{
    size_t a = strlen(parent), b = strlen(name);
    bool slash = a > 0 && parent[a - 1] != '/';
    if (a > SIZE_MAX - b - 2) {
        fputs("ctree: path too long\n", stderr);
        exit(EXIT_FAILURE);
    }
    char *path = allocate(a + b + 2);
    memcpy(path, parent, a);
    if (slash) path[a++] = '/';
    memcpy(path + a, name, b + 1);
    return path;
}

/* Keep every entry on one line, even when a filename contains terminal controls. */
static void print_name(const char *name)
{
    for (const unsigned char *p = (const unsigned char *)name; *p; ++p) {
        if (*p < 32 || *p == 127 || *p == '\\')
            printf("\\x%02x", (unsigned)*p);
        else
            putchar(*p);
    }
}

static void print_error(int error)
{
    printf(" [error: %s]", strerror(error));
    failed = 1;
}

static Entry inspect(const char *path)
{
    Entry e = {0};
    if (lstat(path, &e.info) < 0) {
        e.error = errno;
        return e;
    }
    e.symlink = S_ISLNK(e.info.st_mode);
    if (e.symlink && opt.follow) {
        struct stat target;
        if (stat(path, &target) == 0)
            e.info = target;
        else
            e.error = errno;
    }
    e.directory = S_ISDIR(e.info.st_mode);
    return e;
}

static void print_link(const char *path)
{
    size_t capacity = 128;
    char *target;
    for (;;) {
        target = allocate(capacity + 1);
        ssize_t n = readlink(path, target, capacity);
        if (n < 0) {
            int error = errno;
            free(target);
            print_error(error);
            return;
        }
        if ((size_t)n < capacity) {
            target[n] = '\0';
            break;
        }
        free(target);
        if (capacity > SIZE_MAX / 2 - 1) {
            fputs("ctree: symbolic link target too long\n", stderr);
            exit(EXIT_FAILURE);
        }
        capacity *= 2;
    }
    fputs(" -> ", stdout);
    print_name(target);
    free(target);
}

static void print_entry(const char *path, const char *label, const Entry *entry)
{
    print_name(label);
    if (opt.classify) {
        if (entry->symlink) putchar('@');
        else if (entry->directory) putchar('/');
        else if (S_ISFIFO(entry->info.st_mode)) putchar('|');
        else if (S_ISSOCK(entry->info.st_mode)) putchar('=');
        else if (S_ISREG(entry->info.st_mode) && (entry->info.st_mode & 0111))
            putchar('*');
    }
    if (entry->symlink) print_link(path);
    if (entry->error) print_error(entry->error);
}

static int compare_entries(const void *left, const void *right)
{
    const Entry *a = left, *b = right;
    if (opt.dirs_first && a->directory != b->directory)
        return a->directory ? -1 : 1;
    return strcmp(a->name, b->name);
}

static bool is_ancestor(const Entry *e, const Ancestor *a)
{
    for (; a; a = a->parent)
        if (a->device == e->info.st_dev && a->inode == e->info.st_ino)
            return true;
    return false;
}

/* The caller has printed the directory label, but not its final newline. */
static void walk(const char *path, const char *prefix, unsigned long depth,
                 const Ancestor *ancestors)
{
    if (depth >= opt.max_depth) {
        putchar('\n');
        return;
    }
    DIR *dir = opendir(path);
    if (!dir) {
        print_error(errno);
        putchar('\n');
        return;
    }
    Entry *entries = NULL;
    size_t count = 0, capacity = 0;
    int read_error = 0;
    for (;;) {
        errno = 0;
        struct dirent *item = readdir(dir);
        if (!item) {
            read_error = errno;
            break;
        }
        const char *name = item->d_name;
        if (!strcmp(name, ".") || !strcmp(name, "..")) continue;
        if (!opt.all && name[0] == '.') continue;
        if (opt.ignore && fnmatch(opt.ignore, name, 0) == 0) continue;
        char *child = join_path(path, name);
        Entry e = inspect(child);
        free(child);
        if (opt.dirs_only && !e.directory && !e.error) continue;
        if (count == capacity) {
            size_t next = capacity ? capacity * 2 : 32;
            if (next < capacity || next > SIZE_MAX / sizeof(*entries)) {
                fputs("ctree: too many entries\n", stderr);
                exit(EXIT_FAILURE);
            }
            Entry *grown = realloc(entries, next * sizeof(*entries));
            if (!grown) {
                fputs("ctree: out of memory\n", stderr);
                exit(EXIT_FAILURE);
            }
            entries = grown;
            capacity = next;
        }
        e.name = copy_string(name);
        entries[count++] = e;
    }
    if (closedir(dir) < 0 && !read_error) read_error = errno;
    if (read_error) print_error(read_error);
    putchar('\n');
    if (count > 1) qsort(entries, count, sizeof(*entries), compare_entries);

    for (size_t i = 0; i < count; ++i) {
        Entry *e = &entries[i];
        bool last = i + 1 == count;
        char *child = join_path(path, e->name);
        fputs(prefix, stdout);
        fputs(opt.ascii ? (last ? "`-- " : "|-- ")
                        : (last ? "└── " : "├── "), stdout);
        print_entry(child, opt.full_path ? child : e->name, e);
        if (e->directory) ++directories;
        else ++files;

        if (e->directory && !e->error) {
            if (is_ancestor(e, ancestors)) {
                fputs(" [cycle]\n", stdout);
            } else {
                const char *part = opt.ascii ? (last ? "    " : "|   ")
                                             : (last ? "    " : "│   ");
                size_t a = strlen(prefix), b = strlen(part);
                char *next_prefix = allocate(a + b + 1);
                memcpy(next_prefix, prefix, a);
                memcpy(next_prefix + a, part, b + 1);
                Ancestor next = {e->info.st_dev, e->info.st_ino, ancestors};
                walk(child, next_prefix, depth + 1, &next);
                free(next_prefix);
            }
        } else {
            putchar('\n');
        }
        free(child);
        free(e->name);
    }
    free(entries);
}

static void usage(FILE *out)
{
    fputs("Usage: ctree [OPTIONS] [PATH ...]\n"
          "Print a sorted directory tree (default PATH: .).\n\n"
          "  -a, --all           Include hidden entries\n"
          "  -d, --dirs-only     Show only directories\n"
          "  -L, --level N       Limit depth; 0 prints only the root\n"
          "  -I, --ignore GLOB   Exclude matching basenames (quote the glob)\n"
          "  -l, --follow        Follow symbolic links; detect ancestor cycles\n"
          "  -f, --full-path     Print paths relative to each supplied root\n"
          "  -F, --classify      Append type indicators: / @ * | =\n"
          "      --dirsfirst     Sort directories before other entries\n"
          "      --ascii         Use ASCII branch characters\n"
          "      --noreport      Omit the final entry counts\n"
          "  -h, --help          Show this help\n"
          "      --version       Show version\n", out);
}

int main(int argc, char **argv)
{
    enum { ASCII = 256, DIRS_FIRST, NO_REPORT, VERSION };
    static const struct option options[] = {
        {"all", no_argument, NULL, 'a'},
        {"dirs-only", no_argument, NULL, 'd'},
        {"level", required_argument, NULL, 'L'},
        {"ignore", required_argument, NULL, 'I'},
        {"follow", no_argument, NULL, 'l'},
        {"full-path", no_argument, NULL, 'f'},
        {"classify", no_argument, NULL, 'F'},
        {"ascii", no_argument, NULL, ASCII},
        {"dirsfirst", no_argument, NULL, DIRS_FIRST},
        {"noreport", no_argument, NULL, NO_REPORT},
        {"version", no_argument, NULL, VERSION},
        {"help", no_argument, NULL, 'h'},
        {NULL, 0, NULL, 0}
    };
    int c;
    while ((c = getopt_long(argc, argv, "adL:I:lfFh", options, NULL)) != -1) {
        switch (c) {
        case 'a': opt.all = true; break;
        case 'd': opt.dirs_only = true; break;
        case 'l': opt.follow = true; break;
        case 'f': opt.full_path = true; break;
        case 'F': opt.classify = true; break;
        case 'I': opt.ignore = optarg; break;
        case ASCII: opt.ascii = true; break;
        case DIRS_FIRST: opt.dirs_first = true; break;
        case NO_REPORT: opt.report = false; break;
        case VERSION: puts("ctree 1.0.0"); return EXIT_SUCCESS;
        case 'h': usage(stdout); return EXIT_SUCCESS;
        case 'L': {
            char *end;
            errno = 0;
            unsigned long n = strtoul(optarg, &end, 10);
            if (errno || !*optarg || *end || strspn(optarg, "0123456789") != strlen(optarg)) {
                fprintf(stderr, "ctree: invalid depth: %s\n", optarg);
                return EXIT_FAILURE;
            }
            opt.max_depth = n;
            break;
        }
        default: usage(stderr); return EXIT_FAILURE;
        }
    }

    int roots = argc - optind;
    for (int i = 0; i < (roots ? roots : 1); ++i) {
        const char *path = roots ? argv[optind + i] : ".";
        if (i) putchar('\n');
        Entry root = inspect(path);
        print_entry(path, path, &root);
        if (root.directory && !root.error) {
            Ancestor ancestor = {root.info.st_dev, root.info.st_ino, NULL};
            walk(path, "", 0, &ancestor);
        } else {
            if (!root.error) ++files;
            putchar('\n');
        }
    }
    if (opt.report) {
        printf("\n%" PRIuMAX " director%s", directories, directories == 1 ? "y" : "ies");
        if (!opt.dirs_only)
            printf(", %" PRIuMAX " file%s", files, files == 1 ? "" : "s");
        putchar('\n');
    }
    if (fflush(stdout) == EOF) {
        fputs("ctree: could not write output\n", stderr);
        failed = 1;
    }
    return failed ? EXIT_FAILURE : EXIT_SUCCESS;
}
