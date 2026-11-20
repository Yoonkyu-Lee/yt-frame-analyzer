# TRACE

Map from source files to the intents they implement. Agents read only the files listed for the
intent they are changing. `gate.py` fails if a changed or new source file is not listed here.

Format: one row per file. Intents are IDs from INTENT.md, comma separated. `none` = accidental
code waiting for a decision (see LEDGER.md).

| File | Intents | Notes |
|---|---|---|
| src/tree.c | I1, I2 | |
| src/legacy_cache.c | none | suspected accidental, decision pending |
