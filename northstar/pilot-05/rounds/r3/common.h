#ifndef FILEUTIL_COMMON_H
#define FILEUTIL_COMMON_H
#define _GNU_SOURCE
#include <sys/stat.h>
#include <stdbool.h>
#include <stddef.h>

extern const char *program;
void init_program(const char *argv0);
void error_path(const char *operation, const char *path);
void *xmalloc(size_t size);
char *xstrdup(const char *s);
char *join_path(const char *left, const char *right);
const char *base_name(const char *path);
char *clean_path(const char *path);
#endif
