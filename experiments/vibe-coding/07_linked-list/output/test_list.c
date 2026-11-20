#ifdef NDEBUG
#error "These tests require assertions; compile without -DNDEBUG."
#endif

#include "list.h"

#include <assert.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

/* Bounded traversal also detects cycles without hanging the tests. */
static void check(const List *list, const int *expected, size_t length)
{
    assert(list->size == length);
    const ListNode *node = list->head;
    const ListNode *last = NULL;
    for (size_t i = 0; i < length; ++i) {
        assert(node != NULL);
        assert(node->value == expected[i]);
        int value = 0;
        assert(list_get(list, i, &value));
        assert(value == expected[i]);
        last = node;
        node = node->next;
    }
    assert(node == NULL);
    assert(list->tail == last);
    assert((list->head == NULL) == (length == 0));
}

static void test_boundaries(void)
{
    List list = LIST_INIT;
    int value = 123;
    size_t index = 456;
    check(&list, NULL, 0);
    assert(!list_pop_front(&list, &value));
    assert(!list_remove_at(&list, 0, &value));
    assert(!list_remove_first(&list, 0));
    assert(!list_get(&list, 0, &value));
    assert(!list_find(&list, 0, &index));
    assert(!list_insert(&list, 1, 0));
    assert(!list_insert(&list, SIZE_MAX, 0));
    assert(value == 123 && index == 456);
    list_reverse(&list);
    list_clear(&list);
    check(&list, NULL, 0);

    assert(list_insert(&list, 0, INT_MIN));
    list_reverse(&list);
    check(&list, (int[]){INT_MIN}, 1);
    assert(!list_get(&list, 0, NULL));
    assert(!list_get(&list, SIZE_MAX, &value));
    assert(!list_remove_at(&list, SIZE_MAX, &value));
    assert(value == 123);
    assert(list_remove_at(&list, 0, &value));
    assert(value == INT_MIN);
    check(&list, NULL, 0);

    assert(list_push_back(&list, INT_MAX));
    assert(list_push_front(&list, 7));
    assert(list_insert(&list, 1, 7));
    assert(list_insert(&list, list.size, -1));
    check(&list, (int[]){7, 7, INT_MAX, -1}, 4);
    assert(list_find(&list, 7, &index) && index == 0);
    assert(list_find(&list, INT_MAX, NULL));
    assert(list_remove_first(&list, 7));
    check(&list, (int[]){7, INT_MAX, -1}, 3);
    assert(list_remove_at(&list, 2, NULL));
    assert(list_push_back(&list, 9));
    check(&list, (int[]){7, INT_MAX, 9}, 3);
    list_reverse(&list);
    check(&list, (int[]){9, INT_MAX, 7}, 3);
    list_reverse(&list);
    check(&list, (int[]){7, INT_MAX, 9}, 3);
    list_clear(&list);
    list_clear(&list);
    check(&list, NULL, 0);
    assert(list_push_back(&list, 42));
    assert(list_pop_front(&list, NULL));
    check(&list, NULL, 0);
}

static uint32_t random_number(void)
{
    static uint32_t state = UINT32_C(0x12345678);
    state ^= state << 13;
    state ^= state >> 17;
    state ^= state << 5;
    return state;
}

static void test_against_array(void)
{
    enum { CAPACITY = 128, STEPS = 10000 };
    int model[CAPACITY];
    size_t length = 0;
    List list = LIST_INIT;

    for (int step = 0; step < STEPS; ++step) {
        unsigned operation = random_number() % 9;
        int value = (int)(random_number() % 21) - 10;
        size_t index = random_number() % (length + 2);
        int removed = 123;
        if (operation <= 2 && length < CAPACITY) {
            if (operation == 0) {
                index = 0;
                assert(list_push_front(&list, value));
            } else if (operation == 1) {
                index = length;
                assert(list_push_back(&list, value));
            } else {
                bool valid = index <= length;
                assert(list_insert(&list, index, value) == valid);
                if (!valid) {
                    check(&list, model, length);
                    continue;
                }
            }
            memmove(model + index + 1, model + index,
                    (length - index) * sizeof *model);
            model[index] = value;
            ++length;
        } else if (operation == 3 || operation == 4) {
            if (operation == 3) {
                index = 0;
                assert(list_pop_front(&list, &removed) == (length > 0));
            } else {
                assert(list_remove_at(&list, index, &removed) == (index < length));
            }
            if (index < length) {
                assert(removed == model[index]);
                memmove(model + index, model + index + 1,
                        (length - index - 1) * sizeof *model);
                --length;
            } else {
                assert(removed == 123);
            }
        } else if (operation == 5) {
            list_reverse(&list);
            for (size_t i = 0; i < length / 2; ++i) {
                int temporary = model[i];
                model[i] = model[length - i - 1];
                model[length - i - 1] = temporary;
            }
        } else if (operation == 6 || operation == 7) {
            size_t first = 0;
            while (first < length && model[first] != value) {
                ++first;
            }
            size_t found = SIZE_MAX;
            assert(list_find(&list, value, &found) == (first < length));
            assert(found == (first < length ? first : SIZE_MAX));
            if (operation == 7) {
                assert(list_remove_first(&list, value) == (first < length));
                if (first < length) {
                    memmove(model + first, model + first + 1,
                            (length - first - 1) * sizeof *model);
                    --length;
                }
            }
        } else if (operation == 8 && step % 97 == 0) {
            list_clear(&list);
            length = 0;
        }
        check(&list, model, length);
    }
    list_clear(&list);
    check(&list, NULL, 0);
}

int main(void)
{
    test_boundaries();
    test_against_array();
    puts("All linked list tests passed (including 10,000 model-based steps).");
    return 0;
}
