/* Same probe against the Codex dynamic array (vibe-coding/08 output/dynarray.c). */
#include <stdio.h>
#include "dynarray.h"

int main(void) {
  DynArray a;
  da_init(&a, sizeof(int));
  for (int i = 0; i < 1000; i++)
    da_push(&a, &i);
  int ok = 1;
  for (int i = 0; i < 1000; i++)
    if (*(int *)da_get(&a, (size_t)i) != i) ok = 0;
  printf("1000 appends from default cap: %s (size=%zu cap=%zu)\n", ok ? "ok" : "WRONG", a.size, a.capacity);

  printf("get out of range returns %s\n", da_get(&a, 5000) ? "pointer" : "NULL");
  da_push(&a, da_get(&a, 0));   /* push an element of the array itself while it may reallocate */
  printf("self-push ok: last=%d\n", *(int *)da_get(&a, a.size - 1));
  da_destroy(&a);
  return 0;
}
