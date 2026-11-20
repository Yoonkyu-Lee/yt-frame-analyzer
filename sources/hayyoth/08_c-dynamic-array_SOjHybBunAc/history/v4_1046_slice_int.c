#include <stdio.h>
#include <stdlib.h>

/* HARD VERSION */

typedef struct {
  size_t len; // such as table have 6 person
  size_t cap; // such as table can have max 12 person
  int *data; // array data
} Slice;

Slice *sl_init(size_t cap) {
  Slice *sl = malloc(sizeof(Slice));
  if (!sl) return NULL;

  sl->len = 0;
  sl->cap = cap;
  sl->data = malloc(sizeof(int) * cap);
  if (!sl->data) {
    free(sl);
    return NULL;
  }

  return sl;
}

void sl_append(Slice *sl, int val) {
  // on max size table
  // so, we need move to big table
  if (sl->len == sl->cap) {
    // you don't want call resize every time call append
    sl->cap = sl->cap * 2;
    int *tmp = realloc(sl->data, sizeof(int) * sl->cap);
    // add err handle

    sl->data = tmp;
  }

  sl->data[sl->len] = val;
  sl->len++;
}

void sl_free(Slice *sl) {
  free(sl->data);
  free(sl);
}

void sl_print(const Slice *sl) {
  for (size_t i = 0; i < sl->len; i++)
    printf("%d ", sl->data[i]);
  puts("");
}

int main(void) {
  Slice *sl = sl_init(3);
  if (!sl) return 1;

  sl_append(sl, 11);
  sl_append(sl, 22);
  sl_append(sl, 33);

  sl_print(sl);

  sl_append(sl, 44);
  sl_print(sl);

  sl_free(sl);
  return 0;
}
