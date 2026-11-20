#include "list.h"

#include <stdint.h>
#include <stdlib.h>

static ListNode *new_node(int value)
{
    ListNode *node = malloc(sizeof *node);
    if (node != NULL) {
        node->value = value;
        node->next = NULL;
    }
    return node;
}

bool list_push_front(List *list, int value)
{
    if (list->size == SIZE_MAX) {
        return false;
    }
    ListNode *node = new_node(value);
    if (node == NULL) {
        return false;
    }
    node->next = list->head;
    list->head = node;
    if (list->tail == NULL) {
        list->tail = node;
    }
    ++list->size;
    return true;
}

bool list_push_back(List *list, int value)
{
    if (list->size == SIZE_MAX) {
        return false;
    }
    ListNode *node = new_node(value);
    if (node == NULL) {
        return false;
    }
    if (list->tail != NULL) {
        list->tail->next = node;
    } else {
        list->head = node;
    }
    list->tail = node;
    ++list->size;
    return true;
}

bool list_insert(List *list, size_t index, int value)
{
    if (index > list->size || list->size == SIZE_MAX) {
        return false;
    }
    if (index == 0) {
        return list_push_front(list, value);
    }
    if (index == list->size) {
        return list_push_back(list, value);
    }
    ListNode *previous = list->head;
    for (size_t i = 1; i < index; ++i) {
        previous = previous->next;
    }
    ListNode *node = new_node(value);
    if (node == NULL) {
        return false;
    }
    node->next = previous->next;
    previous->next = node;
    ++list->size;
    return true;
}

bool list_pop_front(List *list, int *value)
{
    if (list->head == NULL) {
        return false;
    }
    ListNode *node = list->head;
    if (value != NULL) {
        *value = node->value;
    }
    list->head = node->next;
    if (list->head == NULL) {
        list->tail = NULL;
    }
    --list->size;
    free(node);
    return true;
}

bool list_remove_at(List *list, size_t index, int *value)
{
    if (index >= list->size) {
        return false;
    }
    if (index == 0) {
        return list_pop_front(list, value);
    }
    ListNode *previous = list->head;
    for (size_t i = 1; i < index; ++i) {
        previous = previous->next;
    }
    ListNode *node = previous->next;
    if (value != NULL) {
        *value = node->value;
    }
    previous->next = node->next;
    if (list->tail == node) {
        list->tail = previous;
    }
    --list->size;
    free(node);
    return true;
}

bool list_remove_first(List *list, int value)
{
    size_t index;
    return list_find(list, value, &index) && list_remove_at(list, index, NULL);
}

bool list_get(const List *list, size_t index, int *value)
{
    if (index >= list->size || value == NULL) {
        return false;
    }
    const ListNode *node = list->head;
    for (size_t i = 0; i < index; ++i) {
        node = node->next;
    }
    *value = node->value;
    return true;
}

bool list_find(const List *list, int value, size_t *index)
{
    size_t i = 0;
    for (const ListNode *node = list->head; node != NULL; node = node->next) {
        if (node->value == value) {
            if (index != NULL) {
                *index = i;
            }
            return true;
        }
        ++i;
    }
    return false;
}

void list_reverse(List *list)
{
    ListNode *previous = NULL;
    ListNode *node = list->head;
    list->tail = list->head;
    while (node != NULL) {
        ListNode *next = node->next;
        node->next = previous;
        previous = node;
        node = next;
    }
    list->head = previous;
}

void list_clear(List *list)
{
    while (list_pop_front(list, NULL)) {
        /* pop_front releases each node. */
    }
}
