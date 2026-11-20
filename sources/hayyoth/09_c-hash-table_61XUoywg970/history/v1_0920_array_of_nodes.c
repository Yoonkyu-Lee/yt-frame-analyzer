#include <stdio.h>

typedef struct Node {
  int value;
  int key;
  struct Node *next;
} Node;

int main(void) {
  Node arr[5] = {0};

  //arr[2] = 22;
  arr[2] = (Node) {
    .value = 22,
    .key = 2,
    .next = NULL
  };

  //arr[7 % 5] = 77;
  arr[2].next = &(Node) {
    .value = 77,
    .key = 7,
    .next = NULL
  };

  return 0;
}
