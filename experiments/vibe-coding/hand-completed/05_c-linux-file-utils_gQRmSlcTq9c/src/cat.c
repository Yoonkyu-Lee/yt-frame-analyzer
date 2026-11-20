#include <stdio.h>

int main(int argc, char *argv[]) {
  if (argc == 1) {
    printf("Usage: cat [path]...\n");
    return 0;
  }

  for (int i = 1; i < argc; i++) {
    const char *path = argv[i];
    FILE *fp = fopen(path, "r");
    if (!fp) {
      perror(path);
      return 1;
    }

    int c; // fgetc give int, char can not hold EOF
    while ((c = fgetc(fp)) != EOF)
      putchar(c);

    fclose(fp);
  }

  return 0;
}
