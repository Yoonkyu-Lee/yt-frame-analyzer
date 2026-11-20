#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define MAX_LINES 1024
#define MAX_LINE_LEN 1024

int mode(char *cmd, char *mode) {
  return (strcmp(cmd, mode) == 0);
}

int main(int argc, char **argv) {
  char cmd[MAX_LINE_LEN];
  char *lines[MAX_LINES];
  int current_line = 0;
  int count_line = 0; // count line in lines

  char filename[255] = {0}; // C add trash value to it
  if (argc > 1) {
    const char *path = argv[1];
    FILE *fp = fopen(path, "r");
    if (!fp) {
      printf("%s: No such file or directory\n", path);
    } else {
      char line[MAX_LINE_LEN];
      while (fgets(line, MAX_LINE_LEN, fp)) {
        line[strcspn(line, "\r\n")] = 0;
        current_line++;
        count_line++;
        lines[current_line] = strdup(line);
      }
    }
    strncpy(filename, path, sizeof(filename));
  }

  int on_edit = 0;
  while (1) {
    if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
    cmd[strcspn(cmd, "\r\n")] = 0;

    if (on_edit) {
      if (mode(cmd, "."))
        on_edit = 0;
      else {
        current_line++;
        lines[current_line] = strdup(cmd);
        count_line++;
      }
    } else {
      if (mode(cmd, "q"))
        break;
      else if (mode(cmd, "a")) // append
        on_edit = 1;
      else if (mode(cmd, "p")) { // print
        if (count_line > 0 )
          puts(lines[current_line]);
        else
          puts("?");
      }
      else if (mode(cmd, "d")) { // delete
        if (count_line > 0) {
          free(lines[current_line]);
          for (int i = current_line; i < count_line; i++)
            lines[i] = lines[i + 1];
          count_line--;
        } else
          puts("?");
      }
      else if (strncmp(cmd, "w", 1) == 0) { // save
        char *path = cmd + 1;
        while (*path == ' ')
          path++;

        if (*path == '\0')
          path = filename;

        if (strlen(path) == 0) {
          puts("?");
          continue;
        }

        FILE *fp = fopen(path, "w");
        if (!fp) {
          perror(path);
          continue;
        }

        for (int i = 1; i <= count_line; i++)
          fprintf(fp, "%s\n", lines[i]);

        if (strlen(filename) == 0)
          strncpy(filename, path, sizeof(filename));

        fclose(fp);
      } else {
        char *endptr;
        int num = strtol(cmd, &endptr, 10);

        if (*endptr == '\0' && num > 0 && num <= count_line) {
          current_line = num;
          puts(lines[current_line]);
        } else {
          puts("?");
        }
      }
    }
  }

  for (int i = 1; i < count_line; i++)
    free(lines[i]);

  return 0;
}
