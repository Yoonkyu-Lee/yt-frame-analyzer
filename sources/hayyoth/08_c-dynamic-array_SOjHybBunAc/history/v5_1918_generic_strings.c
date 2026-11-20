#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/* HARD HARD VERSION - Any data type */

typedef struct {
  size_t len; // such as table have 6 person
  size_t cap; // such as table can have max 12 person
  size_t elm_size;
  void *data; // array data
} Slice;

void *sl_get(const Slice *sl, size_t index) {
  char *data = sl->data;
  return &data[index * sl->elm_size];
}

Slice *sl_init(size_t cap, size_t elm_size) {
  Slice *sl = malloc(sizeof(Slice));
  if (!sl) return NULL;

  sl->len = 0;
  sl->cap = cap;
  sl->elm_size = elm_size;
  sl->data = malloc(elm_size * cap);
  if (!sl->data) {
    free(sl);
    return NULL;
  }

  return sl;
}

void sl_append(Slice *sl, void *val) {
  // on max size table
  // so, we need move to big table
  if (sl->len == sl->cap) {
    // you don't want call resize every time call append
    sl->cap = sl->cap * 2;
    int *tmp = realloc(sl->data, sl->elm_size * sl->cap);
    // add err handle

    sl->data = tmp;
  }

  //sl->data[sl->len] = val;
  memcpy(
      sl_get(sl, sl->len),
      val,
      sl->elm_size
  );

  sl->len++;
}

void sl_free(Slice *sl) {
  free(sl->data);
  free(sl);
}

int main(void) {
  Slice *sl = sl_init(3, sizeof(char *));
  if (!sl) return 1;

  sl_append(sl, &(char *){"Hello"});
  sl_append(sl, &(char *){"World"});
  sl_append(sl, &(char *){"Hayyoth"});

  for (size_t i = 0; i < sl->len; i++)
    printf("%s ", *(char **)sl_get(sl, i));
  puts("");

  sl_free(sl);
  return 0;
}
