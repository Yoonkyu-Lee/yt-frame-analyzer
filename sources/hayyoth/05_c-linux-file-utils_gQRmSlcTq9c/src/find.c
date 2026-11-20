#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>
#include <dirent.h>
#include <string.h>

int walkfind(const char *path, const char *pattern) {
  struct stat st;
  if (lstat(path, &st)) {
    perror(path);
    return 1;
  }

  if (!S_ISDIR(st.st_mode)) {
    if (strstr(path, pattern)) {
      printf("%s\n", path);
    }
  }

  DIR *dir = opendir(path);
  if (!dir) {
    perror(path);
    return 1;
  }

  struct dirent *entry;
  while ((entry = readdir(dir)) != NULL) {
    const char *name = entry->d_name;
    if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
      continue;

    size_t path_len = strlen(path) + strlen(name) + 2;
    char new_path[path_len];
    snprintf(new_path, path_len, "%s/%s", path, name);

    struct stat st_child;
    if (lstat(new_path, &st_child)) {
      perror(new_path);
      return 1;
    }

    if (!S_ISDIR(st_child.st_mode)) {
      if (strstr(name, pattern)) {
        printf("%s\n", new_path);
      }
    } else {
      walkfind(new_path, pattern);
    }

  }

  closedir(dir);

  return 0;
}

int main(int argc, char *argv[]) {
  if (argc < 3) {
    printf("Useage: find [patter] [path]...\n");
    return 0;
  }

  const char *pattern = argv[1];
  for (int i = 2; i < argc; i++) {
    const char *path = argv[i];
    if (walkfind(path, pattern))
      return 1;
  }

  return 0;
}
