/* Same benchmarks against the Codex libraries. Build with -DT07 / -DT08 / -DT09 and the library .c. */
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#if defined(T07)
#include "list.h"
#elif defined(T08)
#include "dynarray.h"
#elif defined(T09)
#include "hash_table.h"
#endif

static double now(void) { struct timespec t; clock_gettime(CLOCK_MONOTONIC, &t); return t.tv_sec + t.tv_nsec / 1e9; }

int main(int argc, char **argv) {
  long n = argc > 1 ? atol(argv[1]) : 10000;
  double t0 = now();
#if defined(T07)
  List l = LIST_INIT;
  for (long i = 0; i < n; i++) list_push_back(&l, (int)i);
  double t1 = now();
  long s = 0;
  for (ListNode *p = l.head; p; p = p->next) s += p->value;   /* no fold in the API: walk the nodes */
  printf("07 codex  n=%ld  append %.3fs  fold %.4fs  (sum %ld)\n", n, t1 - t0, now() - t1, s);
  list_clear(&l);
#elif defined(T08)
  DynArray a; da_init(&a, sizeof(int));
  for (long i = 0; i < n; i++) { int v = (int)i; da_push(&a, &v); }
  double t1 = now();
  long s = 0;
  for (long i = 0; i < n; i++) s += *(int *)da_get(&a, (size_t)i);
  printf("08 codex  n=%ld  push %.3fs  read %.4fs  (sum %ld)\n", n, t1 - t0, now() - t1, s);
  da_destroy(&a);
#elif defined(T09)
  char (*keys)[16] = malloc(n * sizeof *keys);
  for (long i = 0; i < n; i++) snprintf(keys[i], 16, "key%ld", i);
  HashTable *t = ht_create();
  t0 = now();
  for (long i = 0; i < n; i++) ht_put(t, keys[i], (int)i);
  double t1 = now();
  long bad = 0; int v;
  for (long i = 0; i < n; i++) if (!ht_get(t, keys[i], &v) || v != i) bad++;
  printf("09 codex  n=%ld  insert %.3fs  lookup %.3fs  (wrong %ld)\n", n, t1 - t0, now() - t1, bad);
  ht_destroy(t); free(keys);
#endif
  return 0;
}
