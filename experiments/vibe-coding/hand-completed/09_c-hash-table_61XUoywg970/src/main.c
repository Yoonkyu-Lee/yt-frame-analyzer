#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define SIZE 5

typedef struct Node {
  const char *key;
  int value;
  struct Node *next;
} Node;

size_t hash(const char *key) {
  size_t h = 0;

  const char *c = key;
  while (*c) {
    h += *c;
    c++;
  }

  return h % SIZE;
}

void set(Node *arr, const char *key, int value) {
  size_t index = hash(key);
  Node *p = &arr[index];

  // this index is empty, just put value to it
  if (!p->key) {
    p->value = value;
    p->key = key;
    p->next = NULL;
    return;
  }

  // same key already here -> just update value
  for (Node *q = p; q; q = q->next)
    if (strcmp(q->key, key) == 0) {
      q->value = value;
      return;
    }

  // if it not empty -> go to last node and add next
  while (p->next) p = p->next;
  Node *newNode = malloc(sizeof(Node));
  if (!newNode) exit(1);
  newNode->value = value;
  newNode->key = key;
  newNode->next = NULL;
  p->next = newNode;
}

int get(const Node *arr, const char *key) {
  size_t index = hash(key);
  const Node *p = &arr[index];
  for (; p && p->key; p = p->next) // empty cell has no key
    if (strcmp(p->key, key) == 0)
      return p->value;

  return 0;
}

// first node live in arr, only the chain after it is malloc
void destroy(Node *arr) {
  for (size_t i = 0; i < SIZE; i++) {
    Node *p = arr[i].next;
    while (p) {
      Node *next = p->next;
      free(p);
      p = next;
    }
    arr[i].next = NULL;
  }
}

int main(void) {
  Node arr[SIZE] = {0};

  set(arr, "abc", 22);
  set(arr, "cba", 77);
  set(arr, "def", 33);

  printf("%d\n", get(arr, "abc"));
  printf("%d\n", get(arr, "cba"));
  printf("%d\n", get(arr, "def"));

  destroy(arr);
  return 0;
}
