#include <stdio.h>
#include <stdlib.h>

int main(void) {
  size_t cap = 3;
  int *arr = malloc(sizeof(int) * cap);
  if (!arr) return 1;

  arr[0] = 11;
  arr[1] = 22;
  arr[2] = 33;

  for (size_t i = 0; i < cap; i++)
    printf("%d ", arr[i]);
  puts("");

 // how we add 44

  free(arr);
  return 0;
}
