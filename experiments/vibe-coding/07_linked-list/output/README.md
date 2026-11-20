# Linked list in C

A C11 singly linked list of integers, built from individual heap-allocated nodes.
The demo shows insertion, lookup, deletion, reversal, and explicit cleanup.
There are no external dependencies beyond a C compiler.

## Build and run

From PowerShell in this directory, using the compiler inside WSL:

```powershell
wsl.exe -e cc -std=c11 -Wall -Wextra -Wpedantic -Werror main.c list.c -o main
wsl.exe -e ./main
```

Build and run the tests:

```powershell
wsl.exe -e cc -std=c11 -Wall -Wextra -Wpedantic -Werror test_list.c list.c -o test_list
wsl.exe -e ./test_list
```

Check memory usage and undefined behavior with sanitizers:

```powershell
wsl.exe -e cc -std=c11 -Wall -Wextra -Wpedantic -Werror -g '-fsanitize=address,undefined' -fno-omit-frame-pointer test_list.c list.c -o test_list_sanitize
wsl.exe -e ./test_list_sanitize
```

A Makefile is also provided for environments with `make` installed:
`wsl.exe -e make`, `wsl.exe -e make test`, and `wsl.exe -e make sanitize`.

## Usage and ownership

```c
#include "list.h"

List numbers = LIST_INIT;
if (!list_push_back(&numbers, 42)) {
    /* Allocation failed; the list is unchanged. */
}
int value;
if (list_get(&numbers, 0, &value)) {
    /* value is 42. */
}
list_clear(&numbers); /* Release every node before leaving scope. */
```

Initialize each list with `LIST_INIT`. The list owns its nodes; do not copy a
`List` by assignment or change its fields or nodes yourself. Pass its address to
the API. Functions require a valid list pointer. Removed nodes are freed
immediately, so pointers to those nodes become invalid. Reversal relinks existing
nodes without allocating or freeing them. `list_clear` releases all memory and
leaves a reusable empty list.

Indexes start at zero. Insertion accepts indexes from zero through `size`;
lookup and removal require an index below `size`. Boolean operations return
`false` for an invalid index, a missing value, an empty pop, or allocation failure
as applicable. Failed operations preserve the list and output arguments.
Removal outputs and the search index output are optional (`NULL`); the output
of `list_get` is required. Duplicate values are allowed; search and removal by
value use the first match.

| Operation | Time |
| --- | --- |
| Read `size`, `head`, or `tail` | O(1) |
| Push front, push back, pop front | O(1) |
| Insert/remove/get by index | O(n) worst case |
| Find or remove first matching value | O(n) |
| Reverse or clear | O(n) |

The list uses O(n) memory; reversal uses O(1) extra space. A tail pointer makes
appending constant time, but removing the tail still requires finding its
predecessor. Tests cover boundaries, duplicate values, extreme integers,
head/tail consistency, reuse after cleanup, and 10,000 deterministic operations
compared with an array model.
