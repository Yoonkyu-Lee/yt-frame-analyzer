#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <sys/stat.h>

// mkdir that is ok when dir already there
int make(const char *path) {
  if (mkdir(path, 0777) == 0)
    return 0;

  struct stat st;
  if (errno == EEXIST && stat(path, &st) == 0 && S_ISDIR(st.st_mode))
    return 0;
  return 1;
}

// -p: make each parent first, "a/b/c" -> "a", "a/b", "a/b/c"
int make_parents(char *path) {
  for (char *p = path + 1; *p; p++) {
    if (*p != '/')
      continue;
    *p = '\0';
    int err = make(path);
    *p = '/';
    if (err)
      return 1;
  }
  return make(path);
}

int main(int argc, char *argv[]) {
  int i = 1, parents = 0;
  if (argc > 1 && strcmp(argv[1], "-p") == 0) {
    parents = 1;
    i = 2;
  }

  if (i == argc) {
    printf("Usage: mkdir [-p] [path]...\n");
    return 0;
  }

  for (; i < argc; i++) {
    char *path = argv[i];
    if (parents ? make_parents(path) : mkdir(path, 0777)) {
      perror(path);
      return 1;
    }
  }

  return 0;
}
