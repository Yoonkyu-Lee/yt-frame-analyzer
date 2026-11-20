#include <stdio.h>

typedef struct Node {
  int val;
  struct Node *next;
} Node;

void printList(Node *p) {
  for (; p; p->next)
    printf("%d ", p->val);
  printf("\n");
}

int main(void) {
  Node n1 = {.val=10};
  Node n2 = {.val=20};
  Node n3 = {.val=30};

  n1.next = &n2;
  n2.next = &n3;

  printList(&n1);

  return 0;
}
