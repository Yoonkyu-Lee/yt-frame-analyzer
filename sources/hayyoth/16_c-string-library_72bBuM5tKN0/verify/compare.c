/* Compares the author's h* functions (src/main.c) with libc on the same inputs.
 *   gcc -Wall -Wextra -fsanitize=address,undefined -Dmain=author_main verify/compare.c -o cmp && ./cmp
 * main.c is included as-is; its main is renamed by -Dmain=author_main only inside it. */
#include <string.h>
#undef main
#define main author_main
#include "../src/main.c"
#undef main
#include <stdlib.h>

static int fails;
#define CHECK(name, ok) do { if (!(ok)) { printf("FAIL %s\n", name); fails++; } } while (0)

static int sign(int x) { return (x > 0) - (x < 0); }

int main(void) {
    const char *words[] = {"", "a", "hello", "hello world", "hellO", "\xff\x01", "Goats"};
    int n = sizeof words / sizeof *words;

    for (int i = 0; i < n; i++) {
        CHECK("hstrlen", hstrlen(words[i]) == strlen(words[i]));

        char a[64], b[64];
        CHECK("hstrcpy return", hstrcpy(a, (char *)words[i]) == a);
        strcpy(b, words[i]);
        CHECK("hstrcpy bytes", strcmp(a, b) == 0);

        for (int j = 0; j < n; j++) {
            CHECK("hstrcmp sign", sign(hstrcmp(words[i], words[j])) == sign(strcmp(words[i], words[j])));
            char c[64] = "", d[64] = "";
            strcpy(c, words[i]); strcpy(d, words[i]);
            hstrcat(c, words[j]); strcat(d, words[j]);
            CHECK("hstrcat", strcmp(c, d) == 0);
        }
    }

    char m1[16], m2[16];
    hmemcpy(m1, "0123456789abcdef", 16); memcpy(m2, "0123456789abcdef", 16);
    CHECK("hmemcpy", memcmp(m1, m2, 16) == 0);

    /* strncpy: n <= strlen(src) behaves the same (no terminator written by either) */
    char s1[8] = "xxxxxxx", s2[8] = "xxxxxxx";
    hstrncpy(s1, "Goats", 2); strncpy(s2, "Goats", 2);
    CHECK("hstrncpy n<len", memcmp(s1, s2, 8) == 0);

    printf("%d failures in the libc comparison\n", fails);

    /* Known difference: libc strncpy stops at the source terminator and pads with '\0';
       hstrncpy copies n bytes regardless, reading past the end of a short source. */
    if (getenv("PROBE_STRNCPY")) {
        char t[16];
        hstrncpy(t, "Go", 10); /* "Go" is 3 bytes: expect an ASan global-buffer-overflow */
        printf("%s\n", t);
    }
    return fails != 0;
}
