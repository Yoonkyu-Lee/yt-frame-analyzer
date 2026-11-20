/* Same probe against the Codex linked list (vibe-coding/07 output/list.c). */
#include <stdio.h>
#include <time.h>
#include "list.h"

int main(void) {
  List l = LIST_INIT;
  for (int i = 1; i <= 1000; i++)
    list_push_back(&l, i);
  long sum = 0; int v;
  for (size_t i = 0; i < l.size; i++) { list_get(&l, i, &v); sum += v; }
  printf("sum 1..1000 (no map/apply in API, summed via list_get): %ld (expect 500500)\n", sum);

  clock_t t = clock();
  List big = LIST_INIT;
  for (int i = 0; i < 20000; i++)
    list_push_back(&big, i);
  printf("20000 appends: %.3f s\n", (double)(clock() - t) / CLOCKS_PER_SEC);

  int out = -1;
  printf("pop_front on empty list returns %s\n", list_pop_front(&(List)LIST_INIT, &out) ? "true" : "false");
  list_clear(&l);
  list_clear(&big);
  return 0;
}
