/* Same probe against the Codex hash table (vibe-coding/09 output/hash_table.c). */
#include <stdio.h>
#include "hash_table.h"

int main(void) {
  HashTable *t = ht_create();
  ht_put(t, "abc", 22);
  ht_put(t, "abc", 99);
  int v = -1;
  ht_get(t, "abc", &v);
  printf("get abc after put 22 then 99: %d (expect 99), size=%zu (expect 1)\n", v, ht_size(t));

  char keys[200][8];
  for (int i = 0; i < 200; i++) { snprintf(keys[i], 8, "k%d", i); ht_put(t, keys[i], i); }
  int bad = 0;
  for (int i = 0; i < 200; i++) { int x = -1; if (!ht_get(t, keys[i], &x) || x != i) bad++; }
  printf("200 keys: %d wrong\n", bad);

  HashTable *e = ht_create();
  printf("missing key in empty table: %s\n", ht_get(e, "d", &v) ? "found?!" : "not found");
  ht_destroy(e);
  ht_destroy(t);
  return 0;
}
