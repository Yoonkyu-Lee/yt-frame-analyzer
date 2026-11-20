#include "hash_table.h"

#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

#define CHECK(condition) do { \
    if (!(condition)) { \
        fprintf(stderr, "FAIL at line %d: %s\n", __LINE__, #condition); \
        exit(EXIT_FAILURE); \
    } \
} while (0)

static void test_basics(void)
{
    HashTable *table = ht_create();
    CHECK(table != NULL);
    CHECK(ht_size(table) == 0);
    int value = 123;
    CHECK(!ht_get(table, "missing", &value));
    CHECK(value == 123);
    CHECK(!ht_remove(table, "missing"));
    char key[] = "apple";
    CHECK(ht_put(table, key, 0));
    key[0] = 'X'; /* The table owns a copy. */
    CHECK(ht_get(table, "apple", &value) && value == 0);
    CHECK(!ht_contains(table, key));
    CHECK(ht_put(table, "apple", INT_MIN));
    CHECK(ht_size(table) == 1);
    CHECK(ht_get(table, "apple", &value) && value == INT_MIN);
    CHECK(ht_put(table, "", INT_MAX));
    CHECK(ht_get(table, "", &value) && value == INT_MAX);
    CHECK(ht_get(table, "apple", NULL));
    CHECK(ht_remove(table, "apple"));
    CHECK(!ht_remove(table, "apple"));
    CHECK(!ht_contains(table, "apple"));
    CHECK(ht_remove(table, ""));
    CHECK(ht_size(table) == 0);
    CHECK(ht_put(table, "reused", -1));
    CHECK(ht_get(table, "reused", &value) && value == -1);

    CHECK(!ht_put(table, NULL, 1));
    CHECK(!ht_get(table, NULL, &value));
    CHECK(!ht_remove(table, NULL));
    CHECK(!ht_put(NULL, "key", 1));
    CHECK(!ht_get(NULL, "key", &value));
    CHECK(!ht_contains(NULL, "key"));
    CHECK(!ht_remove(NULL, "key"));
    CHECK(ht_size(NULL) == 0);
    ht_destroy(NULL);
    ht_destroy(table);
}

static void test_collisions(void)
{
    /* These FNV-1a hashes all fall in bucket 12 at the initial capacity 16. */
    const char *keys[] = {"a", "q", "A", "Q"};
    HashTable *table = ht_create();
    CHECK(table != NULL);
    for (int i = 0; i < 4; ++i) {
        CHECK(ht_put(table, keys[i], i));
    }
    /* Remove from the middle, tail, and head of the collision chain. */
    CHECK(ht_remove(table, "q"));
    CHECK(ht_remove(table, "a"));
    CHECK(ht_remove(table, "Q"));
    int value;
    CHECK(ht_get(table, "A", &value) && value == 2);
    CHECK(ht_size(table) == 1);
    CHECK(ht_remove(table, "A"));
    CHECK(ht_size(table) == 0);
    ht_destroy(table);
}

static void test_growth(void)
{
    enum { COUNT = 10000 };
    HashTable *table = ht_create();
    CHECK(table != NULL);
    char key[32];
    for (int i = 0; i < COUNT; ++i) {
        snprintf(key, sizeof(key), "key-%d", i);
        CHECK(ht_put(table, key, i));
    }
    CHECK(ht_size(table) == COUNT);
    for (int i = 0; i < COUNT; ++i) {
        int value;
        snprintf(key, sizeof(key), "key-%d", i);
        CHECK(ht_get(table, key, &value) && value == i);
        CHECK(ht_put(table, key, -i));
    }
    CHECK(ht_size(table) == COUNT);
    for (int i = 0; i < COUNT; i += 2) {
        snprintf(key, sizeof(key), "key-%d", i);
        CHECK(ht_remove(table, key));
    }
    CHECK(ht_size(table) == COUNT / 2);
    for (int i = 0; i < COUNT; ++i) {
        int value = 777;
        snprintf(key, sizeof(key), "key-%d", i);
        bool found = ht_get(table, key, &value);
        CHECK(found == (i % 2 != 0));
        CHECK(value == (found ? -i : 777));
    }
    ht_destroy(table); /* Also exercises destruction of a populated table. */
}

int main(void)
{
    test_basics();
    test_collisions();
    test_growth();
    puts("All hash table tests passed.");
    return EXIT_SUCCESS;
}
