#include <stdio.h>

typedef struct Node {
  int val;
  struct Node *next;
} Node;

int main(void) {
  Node n1 = {.val=10};
  Node n2 = {.val=20};
  Node n3 = {.val=30};

  n1.next = &n2;
  n2.next = &n3;


  return 0;
}
