#include <stdio.h>
#include <stdlib.h>

typedef struct Node {
  int val;
  struct Node *next;
} Node;

Node *create(int val) {
  Node *p = malloc(sizeof(Node));
  if (!p) exit(1);

  p->val = val;
  p->next = NULL;
  return p;
}

void append(Node *p, int val) {
  while (p->next) p = p->next; // go to last node
  p->next = create(val);
}

void printList(Node *p) {
  for (; p; p = p->next)
    printf("%d ", p->val);
  printf("\n");
}

int main(void) {
  Node *head = create(10);

  append(head, 20);
  append(head, 30);

  printList(head);

  return 0;
}
