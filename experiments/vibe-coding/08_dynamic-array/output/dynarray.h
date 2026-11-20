#ifndef DYNARRAY_H
#define DYNARRAY_H

#include <stdbool.h>
#include <stddef.h>

/* Fields are public for inspection only; mutate them through this API. */
typedef struct {
    void *data;
    size_t size;
    size_t capacity;
    size_t element_size;
} DynArray;

/* Initialize fresh storage. Do not reinitialize a live array without destroying it. */
bool da_init(DynArray *array, size_t element_size);
/* Releases storage, zeroes the fields, and is safe to call repeatedly. */
void da_destroy(DynArray *array);

/* Allocation failure or size overflow returns false and preserves the array. */
bool da_reserve(DynArray *array, size_t capacity);
bool da_shrink_to_fit(DynArray *array);

/* Copies one element. value may point to an existing element of this array. */
bool da_push(DynArray *array, const void *value);
bool da_insert(DynArray *array, size_t index, const void *value);
bool da_set(DynArray *array, size_t index, const void *value);

/* Returns NULL for an invalid index. See README for pointer lifetime rules. */
void *da_get(DynArray *array, size_t index);
const void *da_get_const(const DynArray *array, size_t index);

bool da_erase(DynArray *array, size_t index);
/* out may be NULL; otherwise it must point to external element-sized storage. */
bool da_pop(DynArray *array, void *out);
/* Discards elements, retaining capacity. Does not free resources inside elements. */
void da_clear(DynArray *array);

#endif
