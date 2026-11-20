#ifndef FILEUTIL_COMMON_H
#define FILEUTIL_COMMON_H
#define _GNU_SOURCE
#include <dirent.h>
#include <getopt.h>
#include <sys/stat.h>
#include <stdbool.h>
#include <stddef.h>

extern const char *program;
enum { RECURSIVE = 1, DIRECTORIES = 2, FORCE = 4, INTERACTIVE = 8, VERBOSE = 16 };
_Noreturn void usage(const char *text, int status);
int next_option(int argc, char **argv, const char *options, const char *help);
int error_path(const char *operation, const char *path);
struct dirent *read_entry(DIR *dir, const char *path, int *failed);
int close_dir(DIR *dir, const char *path);
void *xmalloc(size_t size);
char *xstrdup(const char *s);
char *join_path(const char *left, const char *right);
const char *base_name(const char *path);
char *clean_path(const char *path);
bool confirm(const char *operation, const char *path);
int remove_tree(const char *path, unsigned flags);
int copy_tree(const char *source, const char *target);
int parse_mode(const char *text, mode_t *mode);
#endif
