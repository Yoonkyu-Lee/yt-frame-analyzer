#include <stdio.h>
#include <stdlib.h>
#include <dirent.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>

int color;  // only paint dir blue when output is a terminal
int sorted; // -s: read whole dir and sort by name (cost memory), default is stream

void print_file(const char *name, int is_dir, int indent) {
  if (indent)
    printf("%*c", indent * 2, ' ');

  if (is_dir && color)
    printf("\033[0;34m%s\033[0m\n", name);
  else
    printf("%s\n", name);
}

int walk(const char *path, int indent);

// one entry of dir: print it, and walk into it if it is dir
int visit(const char *path, const struct dirent *dp, int indent) {
  const char *name = dp->d_name;

  // ignore . and ..
  if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
    return 0;

  // walk to sub path
  size_t len = strlen(path) + strlen(name) + 2;
  char sub_path[len];
  snprintf(sub_path, len, "%s/%s", path, name);

  int is_dir = dp->d_type == DT_DIR;
  // some file system not fill d_type, ask lstat
  if (dp->d_type == DT_UNKNOWN) {
    struct stat st;
    is_dir = lstat(sub_path, &st) == 0 && S_ISDIR(st.st_mode);
  }

  print_file(name, is_dir, indent);
  return is_dir ? walk(sub_path, indent + 1) : 0;
}

int walk(const char *path, int indent) {
  int err = 0;

  if (sorted) {
    struct dirent **list;
    int n = scandir(path, &list, NULL, alphasort);
    if (n < 0) {
      perror(path);
      return 1;
    }
    for (int i = 0; i < n; i++) {
      if (visit(path, list[i], indent))
        err = 1;
      free(list[i]);
    }
    free(list);
    return err;
  }

  DIR *dir = opendir(path);
  if (!dir) {
    perror(path);
    return 1;
  }

  struct dirent *dp;
  while ((dp = readdir(dir)) != NULL)
    if (visit(path, dp, indent))
      err = 1;

  closedir(dir);
  return err;
}

int main(int argc, char **argv) {
  int i = 1;
  if (argc > 1 && strcmp(argv[1], "-s") == 0) {
    sorted = 1;
    i = 2;
  }

  const char *path = i < argc ? argv[i] : "."; // default path is current dir

  color = isatty(STDOUT_FILENO);
  int err = walk(path, 0);

  return err;
}
