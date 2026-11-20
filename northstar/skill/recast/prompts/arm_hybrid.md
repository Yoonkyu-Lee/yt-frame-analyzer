# Arm: hybrid (prepend arm_common.md)

Re-derive the CORE, compress the PERIPHERY.

1. Core first: measure the original (own tests + benchmark). In ROUND.md explain the current core
   design (representation, scheduling, persisted state, how completion is detected) and whether
   intent and measurements justify it — look hard at the benchmark's harder scenarios. Decide the
   simplest representation that meets the intents, implement it, and record measured choices
   (e.g. constants you tried and their numbers).
2. Periphery: compress the rest with the compression loop (header per unit, delete what serves no
   intent, rewrite what is longer than its header, verify after each round, stop under 5% gain).
3. Self-skeptic: for each intent ID find the guaranteeing code and an own test; add missing tests.
