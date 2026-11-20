#include <stdio.h>
#include <dirent.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>

int color; // only paint dir blue when output is a terminal

void print_file(const char *name, int is_dir, int indent) {
  if (indent)
    printf("%*c", indent * 2, ' ');

  if (is_dir && color)
    printf("\033[0;34m%s\033[0m\n", name);
  else
    printf("%s\n", name);
}

int walk(const char *path, int indent) {
  DIR *dir = opendir(path);
  if (!dir) {
    perror(path);
    return 1;
  }

  int err = 0;
  struct dirent *dp;
  while ((dp = readdir(dir)) != NULL) {
    const char *name = dp->d_name;

    // ignore . and ..
    if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
      continue;

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
    if (is_dir && walk(sub_path, indent + 1))
      err = 1;
  }

  closedir(dir);
  return err;
}

int main(int argc, char **argv) {
  const char *path = "."; // default path is current dir

  if (argc > 1) {
    path = argv[1];
  }

  color = isatty(STDOUT_FILENO);
  int err = walk(path, 0);

  return err;
}
