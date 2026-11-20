#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>
#include <dirent.h>
#include <string.h>

int walkrm(const char *path) {
  struct stat st;
  if (lstat(path, &st)) {
    perror(path);
    return 1;
  }

  if (!S_ISDIR(st.st_mode)) {
    // it is file (or link), just remove it and done
    if (unlink(path)) {
      perror(path);
      return 1;
    }
    return 0;
  }

  // rm abc --> if is folder , we must remove all file child then remove folder abc
  DIR *dir = opendir(path);
  if (!dir) {
    perror(path);
    return 1;
  }

  int err = 0;
  struct dirent *entry;
  while ((entry = readdir(dir)) != NULL) {
    const char *name = entry->d_name;
    if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
      continue;

    // we have name, now get path
    size_t path_len = strlen(path) + strlen(name) + 2;
    char new_path[path_len];
    snprintf(new_path, path_len, "%s/%s", path, name);

    // file or folder, walkrm know how to remove both
    if (walkrm(new_path))
      err = 1;
  }
  closedir(dir);

  // some child still there, can not remove dir
  if (err)
    return 1;

  // after remove all files child , we remove dir
  if (rmdir(path)) {
    perror(path);
    return 1;
  }

  return 0;
}

int main(int argc, char *argv[]) {
  if (argc == 1) {
    printf("Usage: rm [path]...\n");
    return 0;
  }

  for (int i = 1; i < argc; i++) {
    const char *path = argv[i];
    if (walkrm(path))
      return 1;
  }

  return 0;
}
