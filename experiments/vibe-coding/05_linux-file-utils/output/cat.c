#include "common.h"
#include <errno.h>
#include <getopt.h>
#include <stdio.h>
#include <string.h>

static void usage(void) {
    printf("Usage: %s [-nbsvETAet] [FILE...]\n"
           "Concatenate files (or standard input for - or no files).\n"
           "  -n number lines  -b number nonblank lines  -s squeeze blank lines\n"
           "  -v show controls  -E show line ends  -T show tabs  -A all three\n", program);
}
int main(int argc, char **argv) {
    init_program(argv[0]);
    bool number = false, nonblank = false, squeeze = false;
    bool visible = false, ends = false, tabs = false;
    const struct option options[] = {{"help",0,NULL,1000},{0,0,0,0}};
    int ch;
    while ((ch = getopt_long(argc, argv, "nbsvETAetu", options, NULL)) != -1) {
        switch (ch) {
        case 'n': number = true; break;
        case 'b': nonblank = true; break;
        case 's': squeeze = true; break;
        case 'A': visible = ends = tabs = true; break;
        case 'e': visible = ends = true; break;
        case 't': visible = tabs = true; break;
        case 'v': visible = true; break;
        case 'E': ends = true; break;
        case 'T': tabs = true; break;
        case 'u': break;
        case 1000: usage(); return 0;
        default: usage(); return 1;
        }
    }
    unsigned long long line = 1;
    bool start = true, previous_blank = false, pending_cr = false;
    int failed = 0;
    int count = argc - optind;
    for (int i = 0; i < (count ? count : 1); ++i) {
        const char *path = count ? argv[optind + i] : "-";
        FILE *in = !strcmp(path, "-") ? stdin : fopen(path, "rb");
        if (!in) { error_path("cannot open", path); failed = 1; continue; }
        int c;
        while ((c = fgetc(in)) != EOF) {
            /* -E displays CRLF as ^M$, even without -v. Keep the CR pending
               across file boundaries because input files form one stream. */
            if (pending_cr) {
                if (c == '\n' || visible) fputs("^M", stdout);
                else putchar('\r');
                pending_cr = false;
            }
            if (start) {
                bool blank = c == '\n';
                if (squeeze && blank && previous_blank) continue;
                if ((nonblank && !blank) || (number && !nonblank)) printf("%6llu\t", line++);
                previous_blank = blank;
                start = false;
            }
            if (c == '\n') {
                if (ends) putchar('$');
                putchar('\n'); start = true;
            } else if (c == '\r' && ends) {
                pending_cr = true;
            } else if (c == '\t') {
                if (tabs) fputs("^I", stdout); else putchar(c);
            } else {
                if (visible && c >= 128) { fputs("M-", stdout); c -= 128; }
                if (visible && (c < 32 || c == 127)) {
                    putchar('^'); putchar(c == 127 ? '?' : c + 64);
                } else putchar(c);
            }
            if (ferror(stdout)) break;
        }
        if (ferror(in)) { error_path("cannot read", path); failed = 1; }
        if (in != stdin && fclose(in) != 0) { error_path("cannot close", path); failed = 1; }
        if (ferror(stdout)) break;
    }
    if (pending_cr) {
        if (visible) fputs("^M", stdout); else putchar('\r');
    }
    if (fflush(stdout) == EOF || ferror(stdout)) { error_path("cannot write", "standard output"); failed = 1; }
    return failed;
}
