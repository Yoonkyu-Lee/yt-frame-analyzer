# Generic dynamic array in C

A C11 library built from scratch using `malloc`, `realloc`, and `free`.
It stores fixed-size elements of any
ordinary C type in contiguous heap memory. No external dependencies.

## Build and run from PowerShell

The compiler runs inside WSL; run these commands from this directory:

```powershell
wsl.exe -e cc -std=c11 -Wall -Wextra -Wpedantic -Werror -O2 main.c dynarray.c -o main
wsl.exe -e ./main
wsl.exe -e cc -std=c11 -Wall -Wextra -Wpedantic -Werror -O2 test_dynarray.c dynarray.c -o test_dynarray
wsl.exe -e ./test_dynarray
```

If WSL has `make`, the equivalent commands are:

```powershell
wsl.exe -e make
wsl.exe -e ./main
wsl.exe -e make test
wsl.exe -e make sanitize
```

`make sanitize` runs the tests with AddressSanitizer and UndefinedBehaviorSanitizer.

## Usage

```c
#include "dynarray.h"

int main(void)
{
    DynArray values = {0};
    int number = 42;
    if (!da_init(&values, sizeof(int)) || !da_push(&values, &number)) {
        da_destroy(&values);
        return 1;
    }
    int stored = *(const int *)da_get_const(&values, 0);
    da_destroy(&values);
    return stored == 42 ? 0 : 1;
}
```

Use `sizeof(MyStruct)` for structs or `sizeof(char *)` for string pointers.
Pass the address of the element to push, insert, or set it. All operations copy
the element's bytes; pointer members remain shallow copies. The demo shows
integers and structs, and the tests also cover pointer elements.

## Operations

| Function | Behavior | Time |
| --- | --- | --- |
| `da_init` / `da_destroy` | Initialize / free and zero the array | O(1) |
| `da_push` | Append an element, growing when full | Amortized O(1) |
| `da_insert` | Insert at an index from 0 through size | O(n) |
| `da_get` / `da_get_const` | Obtain a mutable / const element pointer | O(1) |
| `da_set` | Replace an existing element | O(1) |
| `da_erase` | Remove an element and shift following elements left | O(n) |
| `da_pop` | Remove the last element, optionally copying it out | O(1) |
| `da_clear` | Reset size while retaining storage | O(1) |
| `da_reserve` | Ensure capacity for at least the requested count | O(n) if moved |
| `da_shrink_to_fit` | Reduce capacity to size; free storage if empty | O(n) if moved |

Costs assume a fixed element size. Automatic growth starts at four elements and
doubles capacity, capped by the largest representable allocation. `size` counts
live elements; `capacity` counts available slots. Removal does not shrink
automatically, so repeated push/pop operations can reuse allocated storage.

## Contracts and memory safety

- Initialize before use; destroy after use. Initialize fresh or destroyed
  storage only. A zero-initialized array can safely be destroyed.
- Boolean operations return `false` for invalid arguments, allocation failure,
  or size overflow. Failed operations leave a valid array unchanged.
  Accessors return `NULL` for an invalid index.
- Allocation arithmetic is checked before multiplication. `realloc` writes to
  a temporary pointer so allocation failure preserves the existing allocation.
- Push and insert accept a pointer to an existing element, even when growing or
  shifting the array. Other input pointers must reference a complete readable
  element outside the allocation. Set also accepts existing elements.
- Pop's optional output must reference writable element-sized storage outside
  the array. No function frees resources owned by individual elements: clean up
  owned strings, buffers, or other resources yourself before removing them.
- Reallocation invalidates every previously obtained element pointer. Insertion
  and erasure shift elements at and after their index. Removed/cleared elements
  are no longer accessible through the API. Reacquire pointers after mutations.
- Treat the struct's fields as read-only. Do not copy a live `DynArray` by value:
  that would create two owners of one allocation. The library is not synchronized.
- Storage has the alignment provided by `malloc`; over-aligned types requiring
  special allocation are outside the API's scope.

The tests cover empty arrays, bounds, overflow rejection with state preservation,
10,000 appends, overlapping insertion sources, multiple element types, capacity
reuse, and 10,000 deterministic mixed operations checked against a reference.
