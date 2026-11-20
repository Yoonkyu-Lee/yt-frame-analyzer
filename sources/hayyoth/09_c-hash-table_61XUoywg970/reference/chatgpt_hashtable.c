#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SIZE 101

typedef struct Entry {
    char *key;
    int value;
    struct Entry *next;
} Entry;

Entry *table[SIZE];

unsigned long hash(const char *s) {
    unsigned long h = 5381;

    while (*s)
        h = h * 33 + (unsigned char)*s++;

    return h % SIZE;
}

void put(const char *key, int value) {
    unsigned long i = hash(key);

    for (Entry *e = table[i]; e; e = e->next) {
        if (strcmp(e->key, key) == 0) {
            e->value = value;
            return;
        }
    }

    Entry *e = malloc(sizeof *e);
    if (!e) return;

    e->key = malloc(strlen(key) + 1);
    if (!e->key) {
        free(e);
        return;
    }

    strcpy(e->key, key);
    e->value = value;
    e->next = table[i];
    table[i] = e;
}

int *get(const char *key) {
    unsigned long i = hash(key);

    for (Entry *e = table[i]; e; e = e->next)
        if (strcmp(e->key, key) == 0)
            return &e->value;

    return NULL;
}

void destroy(void) {
    for (int i = 0; i < SIZE; i++) {
        Entry *e = table[i];

        while (e) {
            Entry *next = e->next;
            free(e->key);
            free(e);
            e = next;
        }

        table[i] = NULL;
    }
}

int main(void) {
    put("apple", 10);
    put("banana", 20);
    put("orange", 30);
    put("apple", 99);

    int *value = get("apple");

    if (value)
        printf("%d\n", *value);  // 99

    value = get("banana");

    if (value)
        printf("%d\n", *value);  // 20

    destroy();
}
