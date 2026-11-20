#include <stdio.h>
#include <utime.h>

int main(int argc, char *argv[]) {
  if (argc == 1) {
    printf("Usage: touch [path]...\n");
    return 0;
  }

  for (int i = 1; i < argc; i++) {
    const char *path = argv[i];
    // "a" make file if not exist, but never erase old content
    FILE *fp = fopen(path, "a");
    if (!fp) {
      perror(path);
      return 1;
    }
    fclose(fp);

    // set time to now, like real touch
    if (utime(path, NULL)) {
      perror(path);
      return 1;
    }
  }

  return 0;
}
