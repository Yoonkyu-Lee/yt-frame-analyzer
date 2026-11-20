#ifndef LIST_H
#define LIST_H

#include <stdbool.h>
#include <stddef.h>

typedef struct ListNode {
    int value;
    struct ListNode *next;
} ListNode;

typedef struct {
    ListNode *head;
    ListNode *tail;
    size_t size;
} List;

#define LIST_INIT { NULL, NULL, 0 }

/* Initialize with LIST_INIT. A list owns its nodes: never shallow-copy it.
 * Node pointers remain valid until that node is removed or the list is cleared.
 * All functions require a valid List pointer. Do not modify nodes directly.
 * Failed operations leave the list and output parameters unchanged. */

bool list_push_front(List *list, int value);
bool list_push_back(List *list, int value);
/* Insert before index; index == size appends. */
bool list_insert(List *list, size_t index, int value);
/* Output pointers may be NULL when the removed value is not needed. */
bool list_pop_front(List *list, int *value);
bool list_remove_at(List *list, size_t index, int *value);
/* Remove only the first matching value. */
bool list_remove_first(List *list, int value);
/* get requires a non-NULL output; find's index output is optional. */
bool list_get(const List *list, size_t index, int *value);
bool list_find(const List *list, int value, size_t *index);
void list_reverse(List *list);
/* Frees all nodes and restores the empty state. Safe to call repeatedly. */
void list_clear(List *list);

#endif
