# Hash table in C

A dependency-free C11 string-to-int hash table with insert/update, lookup,
membership checks, deletion, and size queries. String keys are copied, so callers
can modify or release their original strings after insertion. Empty keys and all
`int` values are supported.

FNV-1a hashing distributes keys into buckets. Separate chaining handles collisions;
the bucket array doubles before its load factor exceeds 0.75. Operations take
expected O(1) time (insertion amortized), with O(n) worst-case time. Storage is
O(n + total key bytes). Buckets do not shrink after deletion. This hash function
is not intended for adversarial inputs or cryptographic use.

## Build and run (PowerShell with WSL)

Run these commands from this directory:

```powershell
wsl.exe -e cc -std=c11 -Wall -Wextra -Wpedantic -Werror -O2 main.c hash_table.c -o main
wsl.exe -e ./main
wsl.exe -e cc -std=c11 -Wall -Wextra -Wpedantic -Werror -O2 test_hash_table.c hash_table.c -o test_hash_table
wsl.exe -e ./test_hash_table
```

If GNU Make is available, use `wsl.exe -e make` and `wsl.exe -e make test`.
On Linux, run the same compiler or Make commands without `wsl.exe -e`.

The demo inserts three fruits, updates banana's value, looks up a missing key,
and removes apple. Tests cover copied keys, empty keys, integer limits, replacement,
collision-chain deletion, invalid arguments, and growth with 10,000 keys.

## API

Include `hash_table.h` and compile/link `hash_table.c` with your program.

```c
HashTable *table = ht_create();
if (table == NULL) { /* handle allocation failure */ }
else {
    if (!ht_put(table, "score", 42)) { /* handle allocation failure */ }
    int score;
    if (ht_get(table, "score", &score)) { /* use score */ }
    ht_remove(table, "score");
    ht_destroy(table);
}
```

`ht_put` returns false for invalid arguments or allocation failure and preserves
existing contents on failure. `ht_get` returns whether a key exists, optionally
writing its value; a failed lookup leaves the output unchanged. `ht_contains`
checks membership, `ht_remove` returns whether an entry was removed, and `ht_size`
counts entries. NULL tables and keys are handled as documented in the header.
Release every table with `ht_destroy`. Calls on a shared table require external
synchronization.
