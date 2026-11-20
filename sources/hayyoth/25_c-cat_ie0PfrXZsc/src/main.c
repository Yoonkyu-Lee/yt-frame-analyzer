#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <dirent.h>

#define BUFFER_SIZE 1024

void cat(const char *path) {
  printf("\n=========== %s ============\n", path);

  FILE *fp = fopen(path, "r");
  if (!fp) {
    perror(path);
    return;
  }

  char buffers[BUFFER_SIZE];
  size_t nitems;

  while ((nitems = fread(buffers, 1, BUFFER_SIZE, fp)))
    fwrite(buffers, 1, nitems, stdout);

  printf("\n");

  fclose(fp);
}

int is_ignore(const char *path) {
  const char *name = strrchr(path, '/');
  if (name) name++;
  else name = path;

  // ignore all file or folder start with '.'
  if (name[0] == '.')
    return 1;
  return 0;
}

void walkcat(const char *path) {
  if (is_ignore(path))
    return;
  struct stat st;
  if (lstat(path, &st)) {
    perror(path);
    return;
  }

  if (!S_ISDIR(st.st_mode)) {
    // if file -> cat
    cat(path);
    return;
  }

  // is folder
  DIR *dir = opendir(path);
  if (!dir) {
    perror(path);
    return;
  }

  struct dirent *dp;
  while ((dp = readdir(dir)) != NULL) {
    const char *name = dp->d_name;

    if (is_ignore(name))
      continue;

    size_t len = strlen(path) + strlen(name) + 2;
    char sub_path[len];
    snprintf(sub_path, len, "%s/%s", path, name);

    if (dp->d_type == DT_DIR) {
      walkcat(sub_path);
    } else {
      cat(sub_path);
    }
  }
  closedir(dir);
}

int main(int argc, char *argv[]) {
  if (argc == 1) {
    printf("Usage: %s <file_path>...\n", argv[0]);
    return 0;
  }

  for (int i = 1; i < argc; i++) {
    const char *path = argv[i];
    walkcat(path);
  }
  return 0;
}
