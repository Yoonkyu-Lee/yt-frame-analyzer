#include "common.h"
#include <errno.h>
#include <fnmatch.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

enum kind { YES, NO, AND, OR, NOT, NAME, INAME, PATH, TYPE, SIZE, MTIME, MMIN,
            EMPTY, NEWER, PRINT, PRINT0, DELETE, PRUNE };
typedef struct node {
    enum kind kind;
    struct node *left, *right;
    const char *text;
    long long number;
    int comparison;
    long long unit;
    struct timespec stamp;
} Node;
static int position, count, failed;
static char **args;
static int min_depth = 0, max_depth = INT_MAX;
static bool depth_first, has_action;
static time_t started;

static const char help[] =
    "[PATH...] [EXPRESSION]\n"
    "Walk paths without following symlinks (default path: .).\n"
    "Tests: -name GLOB, -iname GLOB, -path GLOB, -type [fdlbcps],\n"
    "       -size [+|-]N[cwbkMG], -mtime [+|-]DAYS, -mmin [+|-]MINUTES,\n"
    "       -empty, -newer FILE, -true, -false\n"
    "Actions: -print, -print0, -delete, -prune\n"
    "Operators: !, -not, -a, -and, -o, -or, ( EXPR )\n"
    "Controls: -mindepth N, -maxdepth N, -depth\n"
    "Adjacent tests imply AND; AND binds before OR. Quote shell globs.\n"
    "Matching paths are printed unless -print, -print0, or -delete appears.\n";
static void syntax(const char *message, const char *token) {
    fprintf(stderr, "%s: %s '%s'\n", program, message, token ? token : "end of expression");
    exit(1);
}
static Node *node(enum kind kind, Node *left, Node *right) {
    Node *n = xmalloc(sizeof *n);
    *n = (Node){ .kind = kind, .left = left, .right = right };
    return n;
}
static const char *argument(const char *option) {
    if (position == count) syntax("missing argument for", option);
    return args[position++];
}
static void number(Node *n, const char *text, bool size) {
    const char *p = text;
    if (*p == '+' || *p == '-') n->comparison = *p++ == '+' ? 1 : -1;
    if (*p < '0' || *p > '9') syntax("invalid number", text);
    char *end;
    errno = 0;
    n->number = strtoll(p, &end, 10);
    if (errno) syntax("number out of range", text);
    n->unit = 512;
    if (size && *end && !end[1]) {
        switch (*end) {
        case 'c': n->unit = 1; break;
        case 'w': n->unit = 2; break;
        case 'b': n->unit = 512; break;
        case 'k': n->unit = 1024; break;
        case 'M': n->unit = 1024 * 1024; break;
        case 'G': n->unit = 1024LL * 1024 * 1024; break;
        default: syntax("invalid size unit", text);
        }
    } else if (*end) syntax("invalid number", text);
}
static Node *parse(bool conjunction);
static Node *parse_unary(void) {
    if (position == count) syntax("expected expression at", NULL);
    const char *token = args[position++];
    if (!strcmp(token, "!") || !strcmp(token, "-not")) return node(NOT, parse_unary(), NULL);
    if (!strcmp(token, "(")) {
        Node *n = parse(false);
        if (position == count || strcmp(args[position++], ")")) syntax("missing closing parenthesis for", "(");
        return n;
    }
    Node *n = node(YES, NULL, NULL);
    if (!strcmp(token, "-name") || !strcmp(token, "-iname") || !strcmp(token, "-path")) {
        n->kind = !strcmp(token, "-name") ? NAME : (!strcmp(token, "-iname") ? INAME : PATH);
        n->text = argument(token);
    } else if (!strcmp(token, "-type")) {
        n->kind = TYPE; n->text = argument(token);
        if (strlen(n->text) != 1 || !strchr("fdlbcps", *n->text)) syntax("invalid type", n->text);
    } else if (!strcmp(token, "-size") || !strcmp(token, "-mtime") || !strcmp(token, "-mmin")) {
        n->kind = !strcmp(token, "-size") ? SIZE : (!strcmp(token, "-mtime") ? MTIME : MMIN);
        number(n, argument(token), n->kind == SIZE);
    } else if (!strcmp(token, "-empty")) n->kind = EMPTY;
    else if (!strcmp(token, "-newer")) {
        struct stat st;
        const char *path = argument(token);
        if (stat(path, &st) < 0) { error_path("cannot stat", path); exit(1); }
        n->kind = NEWER; n->stamp = st.st_mtim;
    } else if (!strcmp(token, "-print") || !strcmp(token, "-print0") || !strcmp(token, "-delete")) {
        has_action = true;
        n->kind = !strcmp(token, "-print") ? PRINT : (!strcmp(token, "-print0") ? PRINT0 : DELETE);
        if (n->kind == DELETE) depth_first = true;
    } else if (!strcmp(token, "-prune")) n->kind = PRUNE;
    else if (!strcmp(token, "-depth")) depth_first = true;
    else if (!strcmp(token, "-mindepth") || !strcmp(token, "-maxdepth")) {
        const char *s = argument(token);
        number(n, s, false);
        if (*s < '0' || *s > '9' || n->number > INT_MAX) syntax("invalid depth", s);
        if (!strcmp(token, "-mindepth")) min_depth = (int)n->number;
        else max_depth = (int)n->number;
    } else if (!strcmp(token, "-false")) n->kind = NO;
    else if (strcmp(token, "-true")) syntax("unknown expression", token);
    return n;
}
static Node *parse(bool conjunction) {
    Node *n = conjunction ? parse_unary() : parse(true);
    while (position < count && strcmp(args[position], ")")) {
        bool disjunction = !strcmp(args[position], "-o") || !strcmp(args[position], "-or");
        if (disjunction == conjunction) break;
        if (disjunction || !strcmp(args[position], "-a") || !strcmp(args[position], "-and")) ++position;
        Node *right = conjunction ? parse_unary() : parse(true);
        n = node(conjunction ? AND : OR, n, right);
    }
    return n;
}
static void free_node(Node *n) {
    if (!n) return;
    free_node(n->left); free_node(n->right); free(n);
}
static bool compare(const Node *n, long long value) {
    return n->comparison > 0 ? value > n->number :
           n->comparison < 0 ? value < n->number : value == n->number;
}
static bool empty(const char *path, const struct stat *st) {
    if (S_ISREG(st->st_mode)) return st->st_size == 0;
    if (!S_ISDIR(st->st_mode)) return false;
    DIR *dir = opendir(path);
    if (!dir) { error_path("cannot open directory", path); failed = 1; return false; }
    int read_failed = 0;
    bool result = !read_entry(dir, path, &read_failed) && !read_failed;
    if (close_dir(dir, path) < 0 || read_failed) failed = 1;
    return result;
}
static bool evaluate(Node *n, const char *path, const struct stat *st, bool *prune) {
    switch (n->kind) {
    case YES: return true;
    case NO: return false;
    case AND: return evaluate(n->left, path, st, prune) && evaluate(n->right, path, st, prune);
    case OR: return evaluate(n->left, path, st, prune) || evaluate(n->right, path, st, prune);
    case NOT: return !evaluate(n->left, path, st, prune);
    case NAME: case INAME: {
        const char *base = !strcmp(path, "/") ? "/" : base_name(path);
        return fnmatch(n->text, base, n->kind == INAME ? FNM_CASEFOLD : 0) == 0;
    }
    case PATH: return fnmatch(n->text, path, 0) == 0;
    case TYPE:
        switch (*n->text) {
        case 'f': return S_ISREG(st->st_mode);
        case 'd': return S_ISDIR(st->st_mode);
        case 'l': return S_ISLNK(st->st_mode);
        case 'b': return S_ISBLK(st->st_mode);
        case 'c': return S_ISCHR(st->st_mode);
        case 'p': return S_ISFIFO(st->st_mode);
        case 's': return S_ISSOCK(st->st_mode);
        }
        return false;
    case SIZE: return compare(n, st->st_size / n->unit + (st->st_size % n->unit != 0));
    case MTIME: case MMIN:
        return compare(n, (long long)floor(difftime(started, st->st_mtime) / (n->kind == MTIME ? 86400 : 60)));
    case EMPTY: return empty(path, st);
    case NEWER: return st->st_mtim.tv_sec > n->stamp.tv_sec ||
                       (st->st_mtim.tv_sec == n->stamp.tv_sec && st->st_mtim.tv_nsec > n->stamp.tv_nsec);
    case PRINT: case PRINT0:
        if (printf("%s%c", path, n->kind == PRINT ? '\n' : '\0') < 0) failed = 1;
        return true;
    case DELETE:
        if ((S_ISDIR(st->st_mode) ? rmdir(path) : unlink(path)) < 0) {
            error_path("cannot delete", path); failed = 1; return false;
        }
        return true;
    case PRUNE: *prune = true; return true;
    }
    return false;
}
static void walk(const char *path, int depth, Node *expression) {
    struct stat st;
    if (lstat(path, &st) < 0) { error_path("cannot stat", path); failed = 1; return; }
    bool prune = false;
    if (!depth_first && depth >= min_depth) evaluate(expression, path, &st, &prune);
    if (S_ISDIR(st.st_mode) && depth < max_depth && !prune) {
        DIR *dir = opendir(path);
        if (!dir) { error_path("cannot open directory", path); failed = 1; }
        else {
            struct dirent *entry;
            while ((entry = read_entry(dir, path, &failed))) {
                char *child = join_path(path, entry->d_name);
                walk(child, depth + 1, expression);
                free(child);
            }
            if (close_dir(dir, path) < 0) failed = 1;
        }
    }
    if (depth_first && depth >= min_depth) evaluate(expression, path, &st, &prune);
}
int main(int argc, char **argv) {
    program = base_name(argv[0]);
    if (argc == 2 && !strcmp(argv[1], "--help")) usage(help, 0);
    int first = 1;
    if (first < argc && !strcmp(argv[first], "-P")) ++first;
    if (first < argc && !strcmp(argv[first], "--")) ++first;
    int end = first;
    while (end < argc && argv[end][0] != '-' && strcmp(argv[end], "!") && strcmp(argv[end], "(")) ++end;
    args = argv; count = argc; position = end;
    Node *expression = position < count ? parse(false) : node(YES, NULL, NULL);
    if (position != count) syntax("unexpected token", argv[position]);
    if (!has_action) expression = node(AND, expression, node(PRINT, NULL, NULL));
    started = time(NULL);
    if (first == end) walk(".", 0, expression);
    else for (int i = first; i < end; ++i) {
        char *path = clean_path(argv[i]);
        walk(path, 0, expression);
        free(path);
    }
    if (fflush(stdout) == EOF || ferror(stdout)) { error_path("cannot write", "standard output"); failed = 1; }
    free_node(expression);
    return failed;
}
