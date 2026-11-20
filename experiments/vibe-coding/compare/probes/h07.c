/* Probe for the human linked list (hand-coding/07 src/main.c, included with main renamed).
   -DCOMPLETED: hand-completed (freeList, empty apply)  -DREMADE: hand-remade (append returns tail, apply is long) */
#define main human_main
#include "h07_src.c"
#undef main
#include <time.h>

#ifdef REMADE
#define APPEND(tail, v) (tail = append(tail, v))
#else
#define APPEND(tail, v) append(tail, v)
#endif

int main(void) {
  setvbuf(stdout, NULL, _IONBF, 0);  /* sanitizers exit early; keep output */
  Node *head = create(1), *tail = head;
  for (int i = 2; i <= 1000; i++)
    APPEND(tail, i);
  printf("sum 1..1000 via apply(add): %ld (expect 500500)\n", (long)apply(head, add));
  map(head, x2);
  printf("after map(x2), apply(add): %ld (expect 1001000)\n", (long)apply(head, add));

  Node *one = create(42);
  printf("single-element apply: %ld (expect 42)\n", (long)apply(one, add));

  clock_t t = clock();
  Node *big = create(0), *btail = big;
  for (int i = 1; i < 20000; i++)
    APPEND(btail, i);
  printf("20000 appends: %.3f s\n", (double)(clock() - t) / CLOCKS_PER_SEC);
#ifdef REMADE
  printf("sum 0..19999 via apply(add): %ld (expect 199990000, int would overflow at 1e5)\n", apply(big, add));
#endif
#ifdef COMPLETED
  freeList(head); freeList(one); freeList(big);
  printf("apply on empty list: %ld (expect 0)\n", (long)apply(NULL, add));
#endif
  (void)tail; (void)btail;
  return 0;  /* original: nodes are never freed, the author left `// add free` */
}
