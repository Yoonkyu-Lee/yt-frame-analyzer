#include "common.h"
#include <stdio.h>
#include <string.h>

/* I2: copy one stream to standard output byte for byte. */
static int copy(FILE *in, const char *path) {
    int c;
    while ((c = getc(in)) != EOF) putchar(c);
    if (ferror(in)) { error_path("cannot read", path); return -1; }
    return 0;
}

/* I2, C1: files in order; no operands or "-" means standard input. */
int main(int argc, char **argv) {
    init_program(argv[0]);
    int failed = 0;
    for (int i = 1; i < argc || i == 1; ++i) {
        const char *path = i < argc ? argv[i] : "-";
        FILE *in = strcmp(path, "-") ? fopen(path, "rb") : stdin;
        if (!in) { error_path("cannot open", path); failed = 1; continue; }
        if (copy(in, path) < 0) failed = 1;
        if (in != stdin) fclose(in);
    }
    if (fflush(stdout) == EOF) { error_path("cannot write", "standard output"); failed = 1; }
    return failed;
}
