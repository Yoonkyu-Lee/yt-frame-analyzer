# Arm: compression loop (prepend arm_common.md)

Do NOT redesign the algorithm — compress. Rounds top-down: module → function → chunk. Each round:
1. one-line header per unit naming the intent ID it serves (none → deletion candidate);
2. compare with INTENT.md and its non-goals; 3. delete, or rewrite from the header;
4. verify with own tests and the benchmark; revert anything that breaks them or regresses the
benchmark noticeably; 5. record in ROUND.md. Stop when a round shrinks the code by less than 5%.
Then a self-skeptic pass: for each intent ID find the code that guarantees it and make sure an own
test pins it; add missing tests.
