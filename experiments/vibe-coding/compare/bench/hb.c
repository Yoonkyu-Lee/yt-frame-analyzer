/* Benchmarks for the human completed data structures (hand-completed 07/08/09 src, main renamed).
   Build with -DT07 / -DT08 / -DT09 and -include of the matching source. */
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

static double now(void) { struct timespec t; clock_gettime(CLOCK_MONOTONIC, &t); return t.tv_sec + t.tv_nsec / 1e9; }

int main(int argc, char **argv) {
  long n = argc > 1 ? atol(argv[1]) : 10000;
  double t0 = now();
#if defined(T07)
  Node *head = create(0);
#ifdef REMADE
  Node *tail = head;
  for (long i = 1; i < n; i++) tail = append(tail, (int)i);
#else
  for (long i = 1; i < n; i++) append(head, (int)i);
#endif
  double t1 = now();
  long s = apply(head, add);
  printf("07 human  n=%ld  append %.3fs  fold %.4fs  (sum %ld)\n", n, t1 - t0, now() - t1, s);
  freeList(head);
#elif defined(T08)
  Slice *sl = sl_init(1, sizeof(int));
  for (long i = 0; i < n; i++) { int v = (int)i; sl_append(sl, &v); }
  double t1 = now();
  long s = 0;
  for (long i = 0; i < n; i++) s += *(int *)sl_get(sl, i);
  printf("08 human  n=%ld  push %.3fs  read %.4fs  (sum %ld)\n", n, t1 - t0, now() - t1, s);
  sl_free(sl);
#elif defined(T09)
  char (*keys)[16] = malloc(n * sizeof *keys);   /* human table does not copy keys: keep them alive */
  for (long i = 0; i < n; i++) snprintf(keys[i], 16, "key%ld", i);
  Node arr[SIZE] = {0};
  t0 = now();
  for (long i = 0; i < n; i++) set(arr, keys[i], (int)i);
  double t1 = now();
  long bad = 0;
  for (long i = 0; i < n; i++) if (get(arr, keys[i]) != i) bad++;
  printf("09 human  n=%ld  insert %.3fs  lookup %.3fs  (wrong %ld)\n", n, t1 - t0, now() - t1, bad);
  destroy(arr); free(keys);
#endif
  return 0;
}
