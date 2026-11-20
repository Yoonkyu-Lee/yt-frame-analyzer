#include <stdio.h>

int main(void) {
  int arr[3];

  arr[0] = 11;
  arr[1] = 22;
  arr[2] = 33;

  for (size_t i = 0; i < 3; i++)
    printf("%d ", arr[i]);
  puts("");

  return 0;
}
