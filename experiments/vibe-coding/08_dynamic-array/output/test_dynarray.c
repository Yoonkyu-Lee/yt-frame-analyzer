#include "dynarray.h"

#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static void test_boundaries(void)
{
    DynArray array = {0};
    int value = 7;
    assert(!da_init(NULL, sizeof(int)));
    assert(!da_init(&array, 0));
    assert(!da_push(&array, &value));
    assert(!da_push(NULL, &value));
    assert(!da_reserve(NULL, 10));
    assert(!da_shrink_to_fit(NULL));
    assert(!da_erase(NULL, 0));
    assert(!da_pop(NULL, NULL));
    assert(da_get(NULL, 0) == NULL);
    assert(da_get_const(NULL, 0) == NULL);
    da_clear(NULL);
    da_destroy(NULL);

    assert(da_init(&array, sizeof(int)));
    assert(array.size == 0 && array.capacity == 0 && array.data == NULL);
    assert(da_reserve(&array, 0));
    assert(!da_pop(&array, &value));
    assert(!da_erase(&array, 0));
    assert(!da_set(&array, 0, &value));
    assert(!da_insert(&array, 1, &value));
    assert(!da_push(&array, NULL));
    assert(da_get(&array, 0) == NULL);
    assert(da_shrink_to_fit(&array));

    assert(da_push(&array, &value));
    void *old_data = array.data;
    size_t old_capacity = array.capacity;
    assert(!da_reserve(&array, SIZE_MAX));
    assert(array.data == old_data && array.capacity == old_capacity);
    assert(array.size == 1 && *(int *)da_get(&array, 0) == 7);
    assert(!da_insert(&array, 2, &value));
    assert(!da_set(&array, 1, &value));
    assert(!da_set(&array, 0, NULL));
    assert(da_get_const(&array, 1) == NULL);
    assert(da_set(&array, 0, da_get(&array, 0)));

    da_clear(&array);
    assert(array.size == 0 && array.capacity == old_capacity);
    assert(da_shrink_to_fit(&array));
    assert(array.data == NULL && array.capacity == 0);
    assert(da_push(&array, &value));
    assert(da_pop(&array, NULL));
    da_destroy(&array);
    assert(array.data == NULL && array.element_size == 0);
    da_destroy(&array);
}

static void test_growth_and_aliasing(void)
{
    DynArray array;
    assert(da_init(&array, sizeof(int)));
    for (int i = 0; i < 10000; ++i) {
        assert(da_push(&array, &i));
        assert(array.capacity >= array.size);
    }
    for (size_t i = 0; i < array.size; ++i) {
        assert(*(const int *)da_get_const(&array, i) == (int)i);
    }
    assert(da_shrink_to_fit(&array));
    assert(array.capacity == array.size);
    /* Appending an internal source when the allocation must grow. */
    assert(da_push(&array, da_get(&array, 23)));
    assert(*(int *)da_get(&array, 10000) == 23);
    da_destroy(&array);

    /* Exercise sources before, at, and after the insertion point, with and
       without growth. Compare against the original sequence. */
    for (size_t reserve = 4; reserve <= 8; reserve += 4) {
        for (size_t index = 0; index <= 4; ++index) {
            for (size_t source = 0; source < 4; ++source) {
                assert(da_init(&array, sizeof(int)));
                assert(da_reserve(&array, reserve));
                for (int i = 0; i < 4; ++i) {
                    assert(da_push(&array, &i));
                }
                assert(da_insert(&array, index, da_get(&array, source)));
                for (size_t i = 0; i < 5; ++i) {
                    int expected = i == index ? (int)source :
                                   (int)(i < index ? i : i - 1);
                    assert(*(int *)da_get(&array, i) == expected);
                }
                da_destroy(&array);
            }
        }
    }
}

static void test_types(void)
{
    typedef struct { int id; double value; char name[16]; } Record;
    DynArray array;
    Record first = {1, 1.25, "first"};
    Record second = {2, 2.5, "second"};
    Record out;
    assert(da_init(&array, sizeof(Record)));
    assert(da_push(&array, &first));
    assert(da_insert(&array, 0, &second));
    assert(da_set(&array, 1, &second));
    assert(da_pop(&array, &out));
    assert(out.id == 2 && out.value == 2.5 && strcmp(out.name, "second") == 0);
    da_destroy(&array);

    const char *words[] = {"generic", "dynamic", "array"};
    assert(da_init(&array, sizeof(const char *)));
    for (size_t i = 0; i < 3; ++i) {
        assert(da_push(&array, &words[i]));
    }
    assert(strcmp(*(const char *const *)da_get_const(&array, 1), "dynamic") == 0);
    da_destroy(&array);
}

static uint32_t next_random(uint32_t *state)
{
    *state = *state * UINT32_C(1664525) + UINT32_C(1013904223);
    return *state;
}

static void test_mixed_operations(void)
{
    DynArray array;
    int reference[256];
    size_t count = 0;
    uint32_t state = 12345;
    assert(da_init(&array, sizeof(int)));
    for (size_t step = 0; step < 10000; ++step) {
        unsigned operation = next_random(&state) % 6;
        int value = (int)(next_random(&state) % 10000);
        if (operation <= 1 && count < 256) {
            size_t index = next_random(&state) % (count + 1);
            memmove(reference + index + 1, reference + index,
                    (count - index) * sizeof(int));
            reference[index] = value;
            count++;
            assert(da_insert(&array, index, &value));
        } else if (operation == 2 && count > 0) {
            size_t index = next_random(&state) % count;
            assert(da_erase(&array, index));
            memmove(reference + index, reference + index + 1,
                    (count - index - 1) * sizeof(int));
            count--;
        } else if (operation == 3 && count > 0) {
            int out;
            assert(da_pop(&array, &out));
            assert(out == reference[--count]);
        } else if (operation == 4 && count > 0) {
            size_t index = next_random(&state) % count;
            assert(da_set(&array, index, &value));
            reference[index] = value;
        } else if (operation == 5) {
            assert(da_shrink_to_fit(&array));
        }
        assert(array.size == count && array.capacity >= count);
        for (size_t i = 0; i < count; ++i) {
            assert(*(int *)da_get(&array, i) == reference[i]);
        }
    }
    da_destroy(&array);
}

int main(void)
{
    test_boundaries();
    test_growth_and_aliasing();
    test_types();
    test_mixed_operations();
    puts("All dynamic array tests passed.");
    return 0;
}
