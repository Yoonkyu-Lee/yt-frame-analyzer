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

void map(Node *p, int (*f)(int)) {
  for (; p; p = p->next)
    p->val = f(p->val);
}

int x2(int x) { return x * 2;}
int x10(int x) { return x * 10;}

int apply(Node *p, int (*f)(int, int)) {
  if (!p) return 0; // empty list, nothing to fold

  int result = p->val;
  for (p = p->next; p; p = p->next)
    result = f(result, p->val);

  return result;
}

int add(int x, int y) { return x + y; }
int sub(int x, int y) { return x - y; }

void freeList(Node *p) {
  while (p) {
    Node *next = p->next; // keep next before p is gone
    free(p);
    p = next;
  }
}

int main(void) {
  Node *head = create(10);

  append(head, 20);
  append(head, 30);

  printList(head);

  printf("sub = %d\n", apply(head, sub));

  freeList(head);
  return 0;
}
