# Segment 6 (10:52 - 13:25) - wj9os_e8Hs4

## Timeline
- [10:52] Firefox window with Google Sheets ("Sheet4"). Memory grid B2:G10: header "Address" | "Memory" (merged over C:F, plus an empty G column), rows address 1..8 with 4-bit patterns (1=0001, 2=0010, 3=0011, 4..8=0000). Column H labels alternate id / name (id, name, id, name, ...). Author is typing "person" in cell I3. Pinned stickies top right: blue (Strang quote), green ("Memory hardware is flat..."). Yellow sticky over the sheet reads "what about struct / we can convention like this". Terminal window ("lab $") is hidden behind the browser at left. Webcam "Human" bottom right.
- [10:58] I4 = "person 1", I6 = "person 2" (I cells span two rows each). Yellow sticky focused, cursor after "like this".
- [10:59] Yellow sticky moved/resized slightly to the right.
- [11:09] Column I filled: person 1 (rows 3-4), person 2 (5-6), person 3 (7-8), person 4 (9-10). Author selects H3:I10 and opens the borders dropdown (Count: 12).
- [11:10] Borders applied: H and I boxed; each person = 2 rows (id + name) grouped under "person N". Selection on J7.
- [11:19] Numbers 1, 2, 3, 4 typed in column G on the "id" rows (G3=1, G5=2, G7=3, G9=4). H4 selected; "Convert to dropdowns" chip shown. Cursor in yellow sticky (text unchanged).
- [11:33] Yellow sticky being rewritten: "so , memory alway flat / we set the rule // (blank) // this is how we get and s" (typing).
- [11:36] Sticky complete: "this is how we get and set value".
- [11:44] Field labels in H being renamed "name" -> "age": H4 = age, H6 = age (row 6 temporarily tall), cursor editing H8 ("nam...").
- [11:45] H4 = age, H6 = name still (editing in progress); H8/H10 name.
- [11:55] Whole table B2:I10 selected (Sum: 50); H10 = "age". Firefox "all tabs" dropdown open: "Blank - Google Sheets", "memory hardware at DuckDuckGo", "C Program for 4-Bit Array Operations - Google Gemini".
- [11:56] Mouse hovers over the Gemini tab entry.
- [11:57] Gemini tab (gemini.google.com/app/d4b657a3667a89...). Previous answer visible: C `get_char` switch code. Empty prompt "Ask Gemini".
- [11:58] Spreadsheet pasted into the prompt (cells flattened): "AddressMemory100011idperson 1 / 20010age300112idperson 2 / 40000age500003idperson 3 / 60000age700004idperson 4 / 80000age".
- [11:59] Blank line added below the pasted text.
- [12:04] Terminal brought to front, zsh tab: history of `cc main.c` / `./a.out` runs of a 4-bit program (Bits: [ 1 0 1 0 ] etc.). Last "Decimal value: 2" highlighted.
- [12:05] Terminal tab switched to "vim main.c" (lines 34-53, cursor 45,13).
- [12:06] Vim scrolled to top (lines 1-21), the earlier bytes[4] program.
- [12:08] Back in Gemini; prompt now has pasted table, blank line and "use " being typed.
- [12:21] Prompt sent (bubble collapsed to 4 lines + chevron); Gemini thinking.
- [12:29] Gemini answer streaming: "Here is the updated C code designed to store person data (such as id, age, and a name like "person 1") using a structure (struct), along with get ..."
- [12:30] "...along with get and set functions to manage them in a simulated memory array." Code block starting.
- [12:31] Code: `#include <string.h>`, `#define MAX_PERSONS 4`, typedef struct Person {int id; int age; char name[20];}.
- [12:32] Scrolled: set_person with memory[index].id/.age and snprintf.
- [12:48] New prompt typed: "No , don't use struct , create state int[] like bytes[]".
- [12:49] Sent; Gemini loading.
- [12:50] Still loading (author scratching head).
- [12:55] Gemini: "Here is the updated C code using a flat int memory[] array as the state instead of a struct. In this approach, each person takes up a fixed block of cells in the flat integer array (e.g., 3 cells per person: Index 0 for ..." streaming.
- [12:56] Text completed ("... Index 1 for age, and Index 2 for a character/name code).").
- [12:57] Code begins: flat `int memory[12]` + set_memory.
- [12:58] Streaming finished.
- [12:59] Code scrolled; get_memory appears. Mouse over the yellow sticky.
- [13:01] Same; sticky title bar visible.
- [13:02] Sticky focused (text cursor in it).
- [13:03] Same.
- [13:20] Yellow sticky text replaced; new text being typed: "i n..."
- [13:22] Sticky: "i know gemini will write what / before it write // this is good way / understa..." (typing).
- [13:23] Sticky cleared, empty with cursor.
- [13:24] Sticky empty, unfocused; Gemini code view scrolled to top of reply.
- [13:25] Gemini code scrolled further: `set_person(int slot, ...)` and `get_char` visible. Sticky empty.

## Texts on screen

### Blue sticky, 10:52-13:25 (unchanged)
```
If everything is flat, you
can't go wrong.

- Professor Gilbert Strang
```
(attribution highlighted grey)

### Green sticky, 10:52-13:25 (unchanged)
```
Memory hardware is flat
C near hardware
C is flat
----
just convention
```
("just convention" highlighted grey)

### Yellow sticky v1, 10:52-11:19
```
what about struct
we can convention like this
```

### Yellow sticky v2 (final), 11:36-13:03
```
so , memory alway flat
we set the rule

this is how we get and set value
```

### Yellow sticky v3, 13:20-13:22 (typed, deleted by 13:23)
```
i know gemini will write what
before it write

this is good way
understa⟨?⟩   (cut off while typing)
```
13:23-13:25: sticky empty.

### Google Sheets "Sheet4" (state ~11:45-11:55)
Columns: B = Address, C:F = Memory (4 bits), G = typed id number, H = field label, I = person group (merged per two rows).

| Address | Memory | G | H | I |
|---|---|---|---|---|
| 1 | 0 0 0 1 | 1 | id | person 1 (rows 3-4) |
| 2 | 0 0 1 0 |  | age | |
| 3 | 0 0 1 1 | 2 | id | person 2 (rows 5-6) |
| 4 | 0 0 0 0 |  | age (name until ~11:45) | |
| 5 | 0 0 0 0 | 3 | id | person 3 (rows 7-8) |
| 6 | 0 0 0 0 |  | ?? (name at 11:45, renamed to age by the pasted text) | |
| 7 | 0 0 0 0 | 4 | id | person 4 (rows 9-10) |
| 8 | 0 0 0 0 |  | age | |

Header: B2 "Address", C2:F2 merged "Memory". The pasted Gemini text confirms final labels: id, age, id, age, id, age, id, age.

### Gemini prompt 1 (pasted, 11:58-12:08)
```
AddressMemory100011idperson 1
20010age300112idperson 2
40000age500003idperson 3
60000age700004idperson 4
80000age

use ⟨?⟩   (typed after "use "; remainder not visible, sent prompt shows only the 4 collapsed lines)
```

### Gemini prompt 2 (12:48-12:49)
```
No , don't use struct , create state int[] like bytes[]
```

### Gemini replies
12:29-12:30:
```
Here is the updated C code designed to store person data (such as id, age, and a name like "person 1") using a structure (struct), along with get and set functions to manage them in a simulated memory array.
```
12:55-12:56:
```
Here is the updated C code using a flat int memory[] array as the state instead of a struct.

In this approach, each person takes up a fixed block of cells in the flat integer array (e.g., 3 cells per person: Index 0 for id, Index 1 for age, and Index 2 for a character/name code).
```

## Code & shell

### Terminal zsh tab (12:04); tabs "~/lab -- vim main.c" and "~/lab -- -zsh"
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
lab $
```

### vim main.c, lines 34-53 (12:05)
```
34 // (Assuming bytes[0] is the Most Significant Bit / MSB)
35 int get_decimal(void) {
36     int decimal = 0;
37     for (int i = 0; i < 4; i++) {
38         decimal = (decimal << 1) | bytes[i];
39     }
40     return decimal;
41 }
42
43 int main(void) {
44     // Example usage:
45     set_bit(2, 1); // Sets index 0 to 1
46     //set_bit(2, 1); // Sets index 2 to 1
47
48     display_bits(); // Output: Bits: [ 1 0 1 0 ]
49
50     printf("Decimal value: %d\n", get_decimal()); // Output: 10 (
51
52     return 0;
53 }
~
"main.c" 53L, 1317B written                45,13   Bot
```
(on line 45 the "2" is the highlighted cursor cell; line 50 is cut off at the window edge)

### vim main.c, lines 1-21 (12:06)
```
 1 #include <stdio.h>
 2
 3 // RULE: each cell can store 0 or 1
 4 int bytes[4] = {0};
 5
 6 // Function to set a bit at a specific index (0 to 3)
 7 void set_bit(int index, int value) {
 8     if (index >= 0 && index < 4) {
 9         bytes[index] = (value != 0) ? 1 : 0; // Ensure only 0 or
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
"main.c" 53L, 1317B written                1,1     Top
```

### Gemini code block, earlier answer (11:57)
```c
#include <stdio.h>

// Function to map a number to a character
char get_char(int value) {
    switch (value) {
        case 1: return 'a';
        case 2: return 'b';
        case 3: return 'c';
        default:
            return '?'; // Returns '?' or '\0' for any unmapped value
    }
}
```
(followed by heading "Integration Example:")

### Gemini struct version (12:31-12:32, partial)
```c
#include <stdio.h>     // partly cut off
#include <string.h>

#define MAX_PERSONS 4

// Define the Person structure based on your data layout
typedef struct {
    int id;
    int age;
    char name[20];
} Person;

// Simulated memory storage for persons
Person memory[MAX_PERSONS];

// Function to set person data at a specific index (address slot)
void set_person(int index, int id, int age, const char *name) {
    if (index >= 0 && index < MAX_PERSONS) {
        memory[index].id = id;
        memory[index].age = age;
        snprintf(memory[index].name, sizeof(memory[index].name), "%s", name);
        ?? (rest not seen)
```

### Gemini flat version (12:57-13:25, partial)
```c
#include <stdio.h>

// Flat integer array state representing memory cells (no structs)
// Each person occupies 3 cells: [id, age, name_code]
// Total capacity: 4 persons * 3 cells = 12 cells
int memory[12] = {0};

// Function to set a specific memory cell value
void set_memory(int address, int value) {
    if (address >= 0 && address < 12) {
        memory[address] = value;
    } else {
        printf("Error: Address out of bounds (0-11)\n");
    }
}

// Function to get a specific memory cell value
int get_memory(int address) {
    if (address >= 0 && address < 12) {
        return memory[address];
        ?? (rest not seen)

// Helper function to set a person using a slot index (0 to 3)   (comment partly cut off)
void set_person(int slot, int id, int age, int name_code) {
    int base_address = slot * 3;
    set_memory(base_address + 0, id);         // ID
    set_memory(base_address + 1, age);        // Age
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
```

## Observations
- He answers his own sticky question "what about struct?" by arguing that a struct is only a naming convention laid over flat memory. In the sheet each person is a group of consecutive cells; the field names (id, age) live only as labels in column H, while the memory itself is just addresses 1..8.
- Sticky progression: "what about struct / we can convention like this" becomes "so , memory alway flat / we set the rule / this is how we get and set value": memory is always flat, we set the rule (convention), and get/set is how we read and write.
- He pastes the sheet into Gemini and asks for C code. Gemini returns a struct version; he rejects it ("No , don't use struct , create state int[] like bytes[]") and forces a flat int memory[] array, so a person becomes 3 cells at base = slot * 3. He uses AI as a typist, steering it toward his own mental model and his earlier hand-style bytes[4] program (set_bit/get_bit, "RULE: each cell can store 0 or 1").
- At 13:20-13:23 he starts a new sticky ("i know gemini will write what before it write / this is good way understa...") meaning he knows what Gemini will write before it writes, which is a good way to understand; he then deletes it. This suggests he treats AI as a check on his own model rather than as a source of knowledge.
- Screen use: stickies pinned as live notes, Sheets as a visual memory model, Gemini for code generation, vim/terminal showing the previous experiment. Webcam shows only him talking and scratching his head while Gemini loads.
