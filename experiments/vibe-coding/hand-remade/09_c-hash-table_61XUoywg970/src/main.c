#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define SIZE 65536 // fixed table: fast until ~1M keys, after that chain get long

typedef struct Node {
  char *key; // own copy, caller string can go away
  int value;
  struct Node *next;
} Node;

size_t hash(const char *key) {
  size_t h = 5381;

  const char *c = key;
  while (*c) {
    h = h * 33 + (unsigned char)*c; // djb2: "abc" and "cba" no longer same index
    c++;
  }

  return h % SIZE;
}

char *copy(const char *key) {
  char *s = strdup(key);
  if (!s) exit(1);
  return s;
}

void set(Node *arr, const char *key, int value) {
  size_t index = hash(key);
  Node *p = &arr[index];

  // this index is empty, just put value to it
  if (!p->key) {
    p->value = value;
    p->key = copy(key);
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
  newNode->key = copy(key);
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

// first node live in arr, only the chain after it is malloc. every key is our copy
void destroy(Node *arr) {
  for (size_t i = 0; i < SIZE; i++) {
    free(arr[i].key);
    Node *p = arr[i].next;
    while (p) {
      Node *next = p->next;
      free(p->key);
      free(p);
      p = next;
    }
    arr[i] = (Node){0};
  }
}

int main(void) {
  static Node arr[SIZE]; // too big for stack, static start all zero

  set(arr, "abc", 22);
  set(arr, "cba", 77);
  set(arr, "def", 33);

  printf("%d\n", get(arr, "abc"));
  printf("%d\n", get(arr, "cba"));
  printf("%d\n", get(arr, "def"));

  destroy(arr);
  return 0;
}
