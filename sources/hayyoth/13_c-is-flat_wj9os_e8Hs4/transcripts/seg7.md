# Segment 7 of 8 (13:25 - 15:19) - wj9os_e8Hs4

## Timeline
- [13:25] Gemini (gemini.google.com) chat in a browser window; a C code block is scrolled (set_person, get_char). Right side: blue sticky "If everything is flat, you can't go wrong. - Professor Gilbert Strang", green sticky "Memory hardware is flat / C near hardware / C is flat / ---- / just convention", and an EMPTY yellow sticky overlapping the browser. A terminal (vim main.c, status `"main.c" 53L, 1317B written`) is partly hidden behind the browser.
- [13:26-13:28] He scrolls down the Gemini code: get_char switch, display_memory(), printf of "Slot %d (Addr %d-%d) -> ID: ...", start of main() with set_person calls.
- [13:29] Hovers "Copy code" on the Gemini code block and clicks it ("Copied to clipboard" toast).
- [13:29] Terminal comes to front showing the OLD main.c (bytes[4] bit-array program: set_bit/get_bit).
- [13:31] In vim command line types `:e main2` (about to open a new file).
- [13:32] vim shows `"main2.c" [New]` with a template: #include <stdio.h> / int main(void) { / return 0; }.
- [13:33] Buffer emptied (`6 fewer lines; before #1 1 second ago`), cursor on empty line 1 (paste about to happen).
- [13:34] Pasted Gemini code is in the buffer (memory[12] version); command line shows `:w`.
- [13:36] Switches to the `~/lab -- -zsh` tab. History shows earlier runs of the old bit program (Bits: [ 1 0 1 0 ], [ 1 0 0 0 ], [ 0 0 0 1 ], [ 0 0 1 0 ]). He is typing `cc ` at the prompt.
- [13:40] Ran `cc main2.c`, `./a.out`: prints "--- Memory Array State ---" with 4 slots, and "Raw value at address 1 (Age of Person 0): 30".
- [13:42-13:43] Selects (highlights) the 4 Slot lines with the mouse; first the header line is included, then the highlight narrows to the 4 slot lines.
- [13:44] Switches back to the vim tab: main2.c content (`"main2.c" [New] 73L, 2375B written`), top of file with memory[12].
- [13:45-13:46] Drags the Terminal window upward on screen (title bar from y~335 to y~228).
- [13:47] Terminal dragged higher (title at y~118). Yellow sticky shows window chrome (hover state), still empty. vim file now shows to line 26.
- [13:48-13:49] He drags the yellow sticky left over the terminal (to x~745, then x~450), cursor blinking in it, still empty. The webcam "Human" window dims its traffic-light buttons in 13:48 (focus change) only.
- [13:58] Yellow sticky now contains: "memory is just array" / blank / "still flat". Sticky positioned x~790-1155.
- [14:01-14:02] vim scrolls down: lines 13-38 then 15-40 visible (set_person, get_char). Sticky unchanged.
- [14:05] In vim, visual mode (`-- VISUAL --`) selects lines 28-31 (body of set_person: base_address and the 3 set_memory calls), 4 lines.
- [14:14] Sticky gets a third entry typed live: "just how we get and set |" (cursor after "set"). Vim still in VISUAL.
- [14:15-14:16] Switches to the browser (Gemini) again, sticky unchanged, then hovers "New chat".
- [14:17] Browser now shows Google Sheets (docs.google.com/spreadsheets/..., tab "Sheet4") with the Address/Memory grid selected (rows 2-10, Sum: 50). Sticky overlaps right part of the grid.
- [14:19] Cell G5 selected (value 2). Sticky has cursor after "set ".
- [14:20] Column I now has "person 1" (rows 3-4) and "person 2" (rows 5-6). Sticky moved down a little (x~800-1165, y~337-525).
- [14:24] Sticky resized taller (y~337-613).
- [14:32] Sticky was cleared and retyped: "so under the hood" / blank / "when we create vari" (typing).
- [14:49] Sticky text: "so under the hood / when we create variable in C / like int , char, float, struct / compiler will convention that" (all selected/highlighted gray).
- [14:51-14:53] He deletes everything; sticky is empty with cursor; he drags it (to x~765, then x~670) and it now covers rows 6-10 of the sheet.
- [15:04] Sticky: "philosophy" / blank / "Memory hardware is flat" / "C near hardware" / "C is flat |". The green sticky's first 3 lines are highlighted gray (he copied from it).
- [15:14] Sticky scrolled: "Memory hardware is flat / C near hardware / **C is flat** / blank / **-----** / blank / **C is flat**" and a new empty line with cursor. ("philosophy" scrolled off.)
- [15:16] Sticky: ... "C is flat" (bold) then a new line starting with "C" (typing).
- [15:19] Sticky: "C is flat" / "C love flat" (both bold), blank line below with cursor.

## Texts on screen

### Blue sticky (13:25-15:19, unchanged)
```
If everything is flat, you
can't go wrong.

- Professor Gilbert Strang
```
(the name "Professor Gilbert Strang" is highlighted gray)

### Green sticky (13:25-15:19, unchanged; "just convention" highlighted gray until ~15:04, then first 3 lines highlighted instead)
```
Memory hardware is flat
C near hardware
C is flat
----
just convention
```

### Yellow sticky, version A (13:58-14:05)
```
memory is just array

still flat
```

### Yellow sticky, version B (14:14-14:24)
```
memory is just array

still flat

just how we get and set
```

### Yellow sticky, version C (typed 14:32, final 14:49; deleted by 14:51)
```
so under the hood

when we create variable in C
like int , char, float, struct

compiler will convention that
```
(spelling "convention" as on screen; at 14:32 only "so under the hood / when we create vari" was typed)

### Yellow sticky, version D (15:04-15:19)
```
philosophy

Memory hardware is flat
C near hardware
C is flat      (bold from 15:14)

-----          (bold)

C is flat      (bold)
C love flat    (bold)
```
(After 15:14 "philosophy" is scrolled out of view. Text may continue below what is visible.)

### Google Sheets "Sheet4" (14:17-15:19; rows 6-10 hidden behind sticky from ~14:53)
Header row 2: B="Address", C-F merged header "Memory".
| Row | B (Address) | C | D | E | F | G | H | I |
|---|---|---|---|---|---|---|---|---|
| 3 | 1 | 0 | 0 | 0 | 1 | 1 | id | (person 1 block, rows 3-4) |
| 4 | 2 | 0 | 0 | 1 | 0 | (blank) | age | person 1 |
| 5 | 3 | 0 | 0 | 1 | 1 | 2 | id | (person 2 block, rows 5-6) |
| 6 | 4 | 0 | 0 | 0 | 0 | (blank) | age | person 2 |
| 7 | 5 | 0 | 0 | 0 | 0 | 3 | id | ?? (hidden by sticky; blank at 14:17) |
| 8 | 6 | 0 | 0 | 0 | 0 | (blank) | age | ?? |
| 9 | 7 | 0 | 0 | 0 | 0 | 4 | id | ?? |
| 10 | 8 | 0 | 0 | 0 | 0 | (blank) | age | ?? |

(At 14:17 column I was empty; "person 1"/"person 2" appeared by 14:20. Status bar showed Sum: 50 at 14:17.)

## Code & shell

### Gemini code block (13:25-13:29), visible fragments
```c
void set_person(int slot, int id, int age, int name_code) {
    int base_address = slot * 3;
    set_memory(base_address + 0, id);        // ID
    set_memory(base_address + 1, age);       // Age
    set_memory(base_address + 2, name_code); // Name / Character code
}

// Helper function to map a number to a character (from your previous requirement)
char get_char(int value) {
    switch (value) {
        case 1: return 'a';
        case 2: return 'b';
        case 3: return 'c';
        default: return '?';
    }
}

// Function to display the memory state
void display_memory(void) {
    printf("\n--- Memory Array State ---\n");
    for (int slot = 0; slot < 4; slot++) {
        int base = slot * 3;
        int id = get_memory(base + 0);
        int age = get_memory(base + 1);
        int code = get_memory(base + 2);

        printf("Slot %d (Addr %d-%d) -> ID: %d, Age: %d, Name Code: %d (Char: %c)\n",
               slot, base, base + 2, id, age, code, get_char(code));
    }
    printf("-------------------------\n");
}

int main(void) {
    // Storing persons into the flat int array state
    set_person(0, 1, 30, 1); // Person 1: ID=1, Age=30, Code=1 ('a')
    set_person(1, 2, 25, 2); // Person 2: ID=2, Age=25, Code=2 ('b')
    set_person(2, 3, 28, 3); // Person 3: ID=3, Age=28, Code=3 ('c')
    set_person(3, 4, 35, 1); // Person 4: ID=4, Age=35, Code=1 ('a')
    ?? (rest of main not seen; program output later shows "Raw value at address 1 (Age of Person 0)")
```

### Old main.c in vim (13:29-13:31), lines 1-21 visible (53 lines total)
```c
 1 #include <stdio.h>
 2
 3 // RULE: each cell can store 0 or 1
 4 int bytes[4] = {0};
 5
 6 // Function to set a bit at a specific index (0 to 3)
 7 void set_bit(int index, int value) {
 8     if (index >= 0 && index < 4) {
 9         bytes[index] = (value != 0) ? 1 : 0; // Ensure only 0 or ?? (cut off)
10     } else {
11         printf("Error: Index out of bounds (0-3)\n");
12     }
13 }
14
15 // Function to get the bit at a specific index
16 int get_bit(int index) {
17     if (index >= 0 && index < 4) {
18         return bytes[index];
19     }
20     printf("Error: Index out of bounds (0-3)\n");
21     return -1;
```
Command line at 13:31: `:e main2`. At 13:32 new-file template:
```c
 1 #include <stdio.h>
 2
 3 int main(void) {
 4
 5   return 0;
 6 }
```
status `"main2.c" [New]   4,2   All`; then buffer cleared, then paste.

### Pasted content in vim (13:34 onward; status later `"main2.c" [New] 73L, 2375B written`), lines 1-40 seen
```c
 1 #include <stdio.h>
 2
 3 // Flat integer array state representing memory cells (no structs ?? (cut off)
 4 // Each person occupies 3 cells: [id, age, name_code]
 5 // Total capacity: 4 persons * 3 cells = 12 cells
 6 int memory[12] = {0};
 7
 8 // Function to set a specific memory cell value
 9 void set_memory(int address, int value) {
10     if (address >= 0 && address < 12) {
11         memory[address] = value;
12     } else {
13         printf("Error: Address out of bounds (0-11)\n");
14     }
15 }
16
17 // Function to get a specific memory cell value
18 int get_memory(int address) {
19     if (address >= 0 && address < 12) {
20         return memory[address];
21     }
22     printf("Error: Address out of bounds (0-11)\n");
23     return -1;
24 }
25
26 // Helper function to set a person using a slot index (0 to 3)
27 void set_person(int slot, int id, int age, int name_code) {
28     int base_address = slot * 3;
29     set_memory(base_address + 0, id);        // ID
30     set_memory(base_address + 1, age);       // Age
31     set_memory(base_address + 2, name_code); // Name / Character
32 }
33
34 // Helper function to map a number to a character (from your prev ?? (cut off)
35 char get_char(int value) {
36     switch (value) {
37         case 1: return 'a';
38         case 2: return 'b';
39         case 3: return 'c';
40         default: return '?';
```
At 14:05 lines 28-31 are visually selected (VISUAL mode, 4 lines).

### Terminal ~/lab -- -zsh (13:36 and 13:40)
```
lab $ cc main.c
lab $ ./a.out
Bits: [ 1 0 1 0 ]
Bit at index 2: 1
Decimal value: 10
lab $ cc main.c
lab $ ./a.out
Bits: [ 1 0 0 0 ]
Decimal value: 8
lab $ cc main.c
lab $ ./a.out
Bits: [ 0 0 0 1 ]
Decimal value: 1
lab $ cc main.c
lab $ ./a.out
Bits: [ 0 0 1 0 ]
Decimal value: 2
lab $ cc main2.c
lab $ ./a.out

--- Memory Array State ---
Slot 0 (Addr 0-2) -> ID: 1, Age: 30, Name Code: 1 (Char: a)
Slot 1 (Addr 3-5) -> ID: 2, Age: 25, Name Code: 2 (Char: b)
Slot 2 (Addr 6-8) -> ID: 3, Age: 28, Name Code: 3 (Char: c)
Slot 3 (Addr 9-11) -> ID: 4, Age: 35, Name Code: 1 (Char: a)
-------------------------

Raw value at address 1 (Age of Person 0): 30
lab $
```
(At 13:36 the last line was `lab $ cc ` being typed.)

## Observations
- He bridges his sheet-based memory model (Address / Memory bits / id, age, person 1, person 2) to real C. He had Gemini write a C program that models "memory" as one flat `int memory[12]` array, with `set_memory(address, value)` / `get_memory(address)` and `set_person(slot, id, age, name_code)` storing each person in 3 consecutive cells (base = slot*3). He pastes it into vim as main2.c (keeping the earlier bit-array main.c), compiles with `cc`, runs it, and highlights that a "person" is just cells 0-2, 3-5, ... and that "Raw value at address 1" is 30, the age.
- The yellow sticky is his live thinking-out-loud note: "memory is just array / still flat / just how we get and set" (a record/struct is just a convention for how we read and write cells). He then starts "so under the hood / when we create variable in C like int, char, float, struct / compiler will convention that" (types and structs are compiler conventions over flat memory) but deletes it and rewrites a new note.
- The new note is headed "philosophy": he copies the green sticky ("Memory hardware is flat / C near hardware / C is flat"), bolds "C is flat", adds "-----" and "C is flat / C love flat" (wordplay on "see/C"). Thesis: hardware is flat, C is near hardware, so C is flat; everything above (types, structs) is convention. Strang quote on the blue sticky: "If everything is flat, you can't go wrong."
- Style: no slides; Gemini output, vim, zsh, Google Sheets and sticky notes share one screen with windows dragged around. He uses AI as a generator of small throwaway demos that he then runs himself in the terminal, and uses stickies to state each claim in minimal words, rewriting until short.
