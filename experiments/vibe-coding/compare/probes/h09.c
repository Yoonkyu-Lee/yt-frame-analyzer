/* Probe for the human hash table (hand-coding/09 src/main.c, included with main renamed). */
#define main human_main
#include "h09_src.c"
#undef main

int main(void) {
  setvbuf(stdout, NULL, _IONBF, 0);  /* sanitizers exit early; keep output */
  Node arr[SIZE] = {0};
  set(arr, "abc", 22);
  set(arr, "abc", 99);  /* same key again */
  printf("get abc after set 22 then 99: %d (expect 99)\n", get(arr, "abc"));
  int n = 0;
  for (Node *q = &arr[hash("abc")]; q; q = q->next) n++;
  printf("nodes in bucket of abc: %d (expect 1)\n", n);

  char keys[200][8];
  for (int i = 0; i < 200; i++) { snprintf(keys[i], 8, "k%d", i); set(arr, keys[i], i); }
  int bad = 0;
  for (int i = 0; i < 200; i++) if (get(arr, keys[i]) != i) bad++;
  printf("200 keys in SIZE=%d table: %d wrong\n", SIZE, bad);
  fflush(stdout);
  printf("missing key in empty bucket: %d\n", get((Node[SIZE]){0}, "d"));
#ifdef COMPLETED
  destroy(arr);
#endif
  return 0;
}
