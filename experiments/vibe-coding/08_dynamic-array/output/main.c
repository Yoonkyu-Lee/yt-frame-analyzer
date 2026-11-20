#include "dynarray.h"

#include <stdio.h>
#include <stdlib.h>

typedef struct {
    const char *name;
    double score;
} Student;

int main(void)
{
    DynArray numbers = {0};
    DynArray students = {0};
    if (!da_init(&numbers, sizeof(int)) ||
        !da_init(&students, sizeof(Student))) {
        goto failure;
    }

    puts("Growing an integer array:");
    for (int i = 0; i < 10; ++i) {
        size_t old_capacity = numbers.capacity;
        int value = i * 10;
        if (!da_push(&numbers, &value)) {
            goto failure;
        }
        if (old_capacity != numbers.capacity) {
            printf("  size=%zu, capacity=%zu\n", numbers.size, numbers.capacity);
        }
    }

    int inserted = 99;
    int popped;
    if (!da_insert(&numbers, 2, &inserted) ||
        !da_erase(&numbers, 0) || !da_pop(&numbers, &popped) ||
        !da_shrink_to_fit(&numbers)) {
        goto failure;
    }
    printf("After insert, erase, and pop (popped %d):", popped);
    for (size_t i = 0; i < numbers.size; ++i) {
        printf(" %d", *(const int *)da_get_const(&numbers, i));
    }
    printf("\nsize=%zu, capacity=%zu\n", numbers.size, numbers.capacity);

    Student ada = {"Ada", 98.5};
    Student grace = {"Grace", 99.0};
    if (!da_push(&students, &ada) || !da_push(&students, &grace)) {
        goto failure;
    }
    puts("\nThe same generic array with structs:");
    for (size_t i = 0; i < students.size; ++i) {
        const Student *student = da_get_const(&students, i);
        printf("  %s: %.1f\n", student->name, student->score);
    }

    da_destroy(&students);
    da_destroy(&numbers);
    return EXIT_SUCCESS;

failure:
    fputs("Could not complete an array operation.\n", stderr);
    da_destroy(&students);
    da_destroy(&numbers);
    return EXIT_FAILURE;
}
