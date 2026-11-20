#include "dynarray.h"

#include <stdint.h>
#include <stdlib.h>
#include <string.h>

static bool initialized(const DynArray *array)
{
    return array != NULL && array->element_size != 0;
}

static unsigned char *element(DynArray *array, size_t index)
{
    return (unsigned char *)array->data + index * array->element_size;
}

bool da_init(DynArray *array, size_t element_size)
{
    if (array == NULL || element_size == 0) {
        return false;
    }
    *array = (DynArray){ .element_size = element_size };
    return true;
}

void da_destroy(DynArray *array)
{
    if (array != NULL) {
        free(array->data);
        *array = (DynArray){0};
    }
}

bool da_reserve(DynArray *array, size_t capacity)
{
    if (!initialized(array) || capacity > SIZE_MAX / array->element_size) {
        return false;
    }
    if (capacity <= array->capacity) {
        return true;
    }

    /* Never overwrite the owning pointer until realloc succeeds. */
    size_t bytes = capacity * array->element_size;
    void *new_data = array->data == NULL ? malloc(bytes) : realloc(array->data, bytes);
    if (new_data == NULL) {
        return false;
    }
    array->data = new_data;
    array->capacity = capacity;
    return true;
}

static bool grow(DynArray *array)
{
    if (array->size < array->capacity) {
        return true;
    }
    size_t maximum = SIZE_MAX / array->element_size;
    if (array->size == maximum) {
        return false;
    }

    size_t capacity = array->capacity;
    if (capacity == 0) {
        capacity = maximum < 4 ? maximum : 4;
    } else {
        capacity = capacity > maximum / 2 ? maximum : capacity * 2;
    }
    return da_reserve(array, capacity);
}

bool da_shrink_to_fit(DynArray *array)
{
    if (!initialized(array)) {
        return false;
    }
    if (array->size == 0) {
        free(array->data);
        array->data = NULL;
        array->capacity = 0;
        return true;
    }
    if (array->size == array->capacity) {
        return true;
    }
    void *new_data = realloc(array->data, array->size * array->element_size);
    if (new_data == NULL) {
        return false;
    }
    array->data = new_data;
    array->capacity = array->size;
    return true;
}

bool da_insert(DynArray *array, size_t index, const void *value)
{
    if (!initialized(array) || value == NULL || index > array->size) {
        return false;
    }

    /* Remember an internal source by index before realloc or shifting moves it.
       uintptr_t avoids relational comparisons between unrelated pointers. */
    size_t source = SIZE_MAX;
    if (array->data != NULL) {
        uintptr_t base = (uintptr_t)array->data;
        uintptr_t address = (uintptr_t)value;
        if (address >= base && address - base < array->capacity * array->element_size) {
            size_t offset = (size_t)(address - base);
            if (offset % array->element_size != 0 ||
                offset / array->element_size >= array->size) {
                return false;
            }
            source = offset / array->element_size;
        }
    }
    if (!grow(array)) {
        return false;
    }

    memmove(element(array, index + 1), element(array, index),
            (array->size - index) * array->element_size);
    if (source != SIZE_MAX) {
        value = element(array, source >= index ? source + 1 : source);
    }
    memcpy(element(array, index), value, array->element_size);
    array->size++;
    return true;
}

bool da_push(DynArray *array, const void *value)
{
    return array != NULL && da_insert(array, array->size, value);
}

bool da_set(DynArray *array, size_t index, const void *value)
{
    if (!initialized(array) || index >= array->size || value == NULL) {
        return false;
    }
    memmove(element(array, index), value, array->element_size);
    return true;
}

void *da_get(DynArray *array, size_t index)
{
    if (!initialized(array) || index >= array->size) {
        return NULL;
    }
    return element(array, index);
}

const void *da_get_const(const DynArray *array, size_t index)
{
    if (!initialized(array) || index >= array->size) {
        return NULL;
    }
    return (const unsigned char *)array->data + index * array->element_size;
}

bool da_erase(DynArray *array, size_t index)
{
    if (!initialized(array) || index >= array->size) {
        return false;
    }
    memmove(element(array, index), element(array, index + 1),
            (array->size - index - 1) * array->element_size);
    array->size--;
    return true;
}

bool da_pop(DynArray *array, void *out)
{
    if (!initialized(array) || array->size == 0) {
        return false;
    }
    if (out != NULL) {
        memcpy(out, element(array, array->size - 1), array->element_size);
    }
    array->size--;
    return true;
}

void da_clear(DynArray *array)
{
    if (initialized(array)) {
        array->size = 0;
    }
}
