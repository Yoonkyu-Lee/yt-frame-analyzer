/* Probe for the human dynamic array (hand-coding/08 src/main.c, included with main renamed). */
#define main human_main
#include "h08_src.c"
#undef main

int main(void) {
  setvbuf(stdout, NULL, _IONBF, 0);  /* sanitizers exit early; keep output */
  Slice *sl = sl_init(1, sizeof(int));
  for (int i = 0; i < 1000; i++)
    sl_append(sl, &i);
  int ok = 1;
  for (int i = 0; i < 1000; i++)
    if (*(int *)sl_get(sl, i) != i) ok = 0;
  printf("1000 appends from cap 1: %s (len=%zu cap=%zu)\n", ok ? "ok" : "WRONG", sl->len, sl->cap);
  sl_free(sl);

  Slice *z = sl_init(0, sizeof(int));   /* capacity 0: cap*2 stays 0 */
  int v = 7;
  sl_append(z, &v);
  printf("cap 0 start: len=%zu cap=%zu\n", z->len, z->cap);
  sl_free(z);
  return 0;
}
