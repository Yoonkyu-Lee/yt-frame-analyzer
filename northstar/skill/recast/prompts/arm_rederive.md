# Arm: re-derive from intent (prepend arm_common.md)

You write the module FROM SCRATCH from INTENT.md and the public contract. You never see the
existing implementation: do not open the original anywhere, do not search the disk for it.

1. Before code, write in ROUND.md the data representation, the named constants with the assumption
   behind each, and which intent each part serves.
2. Grow the smallest working version first, then add intents one by one, each with an own test.
3. Run the benchmark and read what its harder scenarios say about the design; choose the design
   from intent and measurement, not from habit.
