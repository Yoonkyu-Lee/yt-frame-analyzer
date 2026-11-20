#include "hash_table.h"

#include <stdio.h>
#include <stdlib.h>

int main(void)
{
    HashTable *table = ht_create();
    if (table == NULL) {
        fputs("Could not allocate hash table.\n", stderr);
        return EXIT_FAILURE;
    }
    if (!ht_put(table, "apple", 10) || !ht_put(table, "banana", 20) ||
        !ht_put(table, "cherry", 30) || !ht_put(table, "banana", 25)) {
        fputs("Could not insert key.\n", stderr);
        ht_destroy(table);
        return EXIT_FAILURE;
    }

    printf("Hash table contains %zu entries.\n", ht_size(table));
    const char *keys[] = {"apple", "banana", "cherry", "missing"};
    for (size_t i = 0; i < sizeof(keys) / sizeof(keys[0]); ++i) {
        int value;
        if (ht_get(table, keys[i], &value)) {
            printf("%s => %d\n", keys[i], value);
        } else {
            printf("%s => (not found)\n", keys[i]);
        }
    }
    if (ht_remove(table, "apple")) {
        printf("Removed apple; %zu entries remain.\n", ht_size(table));
    }
    ht_destroy(table);
    return EXIT_SUCCESS;
}
