#ifndef HASH_TABLE_H
#define HASH_TABLE_H

#include <stdbool.h>
#include <stddef.h>

/* A string-to-int map. Keys are copied and owned by the table.
 * This API is not thread-safe; callers must synchronize shared access. */
typedef struct HashTable HashTable;

/* Returns NULL if allocation fails. */
HashTable *ht_create(void);
void ht_destroy(HashTable *table);
size_t ht_size(const HashTable *table);

/* Inserts or replaces a value. Returns false on invalid input or allocation
 * failure; in either case the table's contents are unchanged. */
bool ht_put(HashTable *table, const char *key, int value);

/* Returns true if found. out_value may be NULL; on failure it is unchanged.
 * Empty keys are valid. NULL tables/keys are treated as absent. */
bool ht_get(const HashTable *table, const char *key, int *out_value);
bool ht_contains(const HashTable *table, const char *key);
bool ht_remove(HashTable *table, const char *key);

#endif
