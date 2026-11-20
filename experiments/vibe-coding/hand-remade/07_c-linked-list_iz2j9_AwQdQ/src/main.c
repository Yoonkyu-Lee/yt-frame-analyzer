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

// give last node, get new last node back: no need walk to the end every time
Node *append(Node *tail, int val) {
  tail->next = create(val);
  return tail->next;
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

// result is long: sum of many int can be bigger than int
long apply(Node *p, long (*f)(long, long)) {
  if (!p) return 0; // empty list, nothing to fold

  long result = p->val;
  for (p = p->next; p; p = p->next)
    result = f(result, p->val);

  return result;
}

long add(long x, long y) { return x + y; }
long sub(long x, long y) { return x - y; }

void freeList(Node *p) {
  while (p) {
    Node *next = p->next; // keep next before p is gone
    free(p);
    p = next;
  }
}

int main(void) {
  Node *head = create(10);

  Node *tail = append(head, 20);
  tail = append(tail, 30);

  printList(head);

  printf("sub = %ld\n", apply(head, sub));

  freeList(head);
  return 0;
}
