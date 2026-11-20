#include "hash_table.h"

#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define INITIAL_CAPACITY 16u

typedef struct Entry {
    char *key;
    int value;
    uint64_t hash;
    struct Entry *next;
} Entry;

struct HashTable {
    Entry **buckets;
    size_t capacity;
    size_t size;
};

/* FNV-1a hashes bytes consistently regardless of whether char is signed. */
static uint64_t hash_key(const char *key)
{
    uint64_t hash = UINT64_C(14695981039346656037);
    const unsigned char *p = (const unsigned char *)key;
    while (*p != '\0') {
        hash ^= *p++;
        hash *= UINT64_C(1099511628211);
    }
    return hash;
}

static Entry *find_entry(const HashTable *table, const char *key, uint64_t hash)
{
    for (Entry *entry = table->buckets[hash % table->capacity];
         entry != NULL; entry = entry->next) {
        if (entry->hash == hash && strcmp(entry->key, key) == 0) {
            return entry;
        }
    }
    return NULL;
}

static bool grow(HashTable *table)
{
    if (table->capacity > SIZE_MAX / 2) {
        return false;
    }
    size_t capacity = table->capacity * 2;
    if (capacity > SIZE_MAX / sizeof(Entry *)) {
        return false;
    }
    Entry **buckets = calloc(capacity, sizeof(*buckets));
    if (buckets == NULL) {
        return false;
    }
    for (size_t i = 0; i < table->capacity; ++i) {
        Entry *entry = table->buckets[i];
        while (entry != NULL) {
            Entry *next = entry->next;
            size_t index = entry->hash % capacity;
            entry->next = buckets[index];
            buckets[index] = entry;
            entry = next;
        }
    }
    free(table->buckets);
    table->buckets = buckets;
    table->capacity = capacity;
    return true;
}

HashTable *ht_create(void)
{
    HashTable *table = malloc(sizeof(*table));
    if (table == NULL) {
        return NULL;
    }
    table->buckets = calloc(INITIAL_CAPACITY, sizeof(*table->buckets));
    if (table->buckets == NULL) {
        free(table);
        return NULL;
    }
    table->capacity = INITIAL_CAPACITY;
    table->size = 0;
    return table;
}

void ht_destroy(HashTable *table)
{
    if (table == NULL) {
        return;
    }
    for (size_t i = 0; i < table->capacity; ++i) {
        Entry *entry = table->buckets[i];
        while (entry != NULL) {
            Entry *next = entry->next;
            free(entry->key);
            free(entry);
            entry = next;
        }
    }
    free(table->buckets);
    free(table);
}

size_t ht_size(const HashTable *table)
{
    return table == NULL ? 0 : table->size;
}

bool ht_put(HashTable *table, const char *key, int value)
{
    if (table == NULL || key == NULL) {
        return false;
    }
    uint64_t hash = hash_key(key);
    Entry *existing = find_entry(table, key, hash);
    if (existing != NULL) {
        existing->value = value;
        return true;
    }

    Entry *entry = malloc(sizeof(*entry));
    if (entry == NULL) {
        return false;
    }
    size_t length = strlen(key);
    if (length == SIZE_MAX) {
        free(entry);
        return false;
    }
    entry->key = malloc(length + 1);
    if (entry->key == NULL) {
        free(entry);
        return false;
    }
    memcpy(entry->key, key, length + 1);

    /* Grow before exceeding a 0.75 load factor; avoid multiplication overflow. */
    if (table->size >= table->capacity - table->capacity / 4 && !grow(table)) {
        free(entry->key);
        free(entry);
        return false;
    }
    size_t index = hash % table->capacity;
    entry->hash = hash;
    entry->value = value;
    entry->next = table->buckets[index];
    table->buckets[index] = entry;
    ++table->size;
    return true;
}

bool ht_get(const HashTable *table, const char *key, int *out_value)
{
    if (table == NULL || key == NULL) {
        return false;
    }
    Entry *entry = find_entry(table, key, hash_key(key));
    if (entry == NULL) {
        return false;
    }
    if (out_value != NULL) {
        *out_value = entry->value;
    }
    return true;
}

bool ht_contains(const HashTable *table, const char *key)
{
    return ht_get(table, key, NULL);
}

bool ht_remove(HashTable *table, const char *key)
{
    if (table == NULL || key == NULL) {
        return false;
    }
    uint64_t hash = hash_key(key);
    Entry **link = &table->buckets[hash % table->capacity];
    while (*link != NULL) {
        Entry *entry = *link;
        if (entry->hash == hash && strcmp(entry->key, key) == 0) {
            *link = entry->next;
            free(entry->key);
            free(entry);
            --table->size;
            return true;
        }
        link = &entry->next;
    }
    return false;
}
