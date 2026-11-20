#include "list.h"

#include <stdio.h>
#include <stdlib.h>

static void print_list(const char *label, const List *list)
{
    printf("%-18s ", label);
    for (const ListNode *node = list->head; node != NULL; node = node->next) {
        printf("%d -> ", node->value);
    }
    printf("NULL (size: %zu)\n", list->size);
}

int main(void)
{
    List list = LIST_INIT;
    if (!list_push_back(&list, 20) || !list_push_back(&list, 30) ||
        !list_push_front(&list, 10) || !list_insert(&list, 2, 25)) {
        fputs("Could not allocate a list node.\n", stderr);
        list_clear(&list);
        return EXIT_FAILURE;
    }

    puts("A singly linked list in C\n");
    print_list("After insertion:", &list);

    size_t index;
    if (list_find(&list, 25, &index)) {
        printf("Found 25 at index %zu\n", index);
    }
    int value;
    if (list_get(&list, 1, &value)) {
        printf("Value at index 1: %d\n", value);
    }

    list_remove_first(&list, 20);
    print_list("After removing 20:", &list);
    list_reverse(&list);
    print_list("After reversal:", &list);
    if (list_pop_front(&list, &value)) {
        printf("Popped: %d\n", value);
    }
    print_list("After pop:", &list);
    list_clear(&list);
    print_list("After cleanup:", &list);
    return EXIT_SUCCESS;
}
