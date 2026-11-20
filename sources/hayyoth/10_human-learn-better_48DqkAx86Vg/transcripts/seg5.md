# Segment 5 of 6 (07:38 - 08:58) - "How we (Human) learn better than AI?"

Note: yellow sticky at top right (collapsed title strip) reads "How we learn better than AI?..." throughout. Webcam "Human" overlay ignored except where noted.

## Timeline
- [07:38] macOS Preview showing vdoc.pdf (Kernighan/Pike "The Practice of Programming"), page 73 of 733, section 2.7 Lists (sidebar: Chapter 2 Algorithms and Data Structures expanded). Code shown: `lookup` (sequential search in a list) and `apply`. Cyan sticky over it reads "it work / AI love complex / your brain not".
- [07:39] PDF scrolled to the `apply` code; cyan sticky unchanged.
- [07:43] Cyan sticky cleared and re-typed: "Hard to understand " (cursor after it).
- [07:44] Mission Control (Desktop 1 / Desktop 2). Windows: Firefox with Gemini tab "Hayyoth Lab Forest Fire Simulation - Google Gemini" (C code: `main`, `hSetup`, `hCell(rule)`, `hRender(view)`), the PDF reader, and an Apple Books style "PDFs" library list (125 Problems in Text Algorithms with Solutions; The Scheme Programming Language, 4th Edition - R. Kent Dybvig; vdoc.pub_the-practice-of-programming - Brian W. Kernighan; Code Biology _ A New Science of Life; Numerical C). Sticky "Hard to understand" visible. Dock visible.
- [07:45] Firefox, tab 1 "The Creation of Chain - Linked List. Conjured in C" (youtube.com/watch?v=iz2j9_AwQdQ&t=123s), his own earlier video on channel Hayyoth (64 subscribers, Analytics / Edit video buttons). Video frame shows a vim terminal with C linked list code plus a Chez Scheme terminal. Sticky "Hard to understand".
- [07:47-07:48] Video zoomed in to the vim `main.c` linked-list code (18 lines). Sticky still "Hard to understand".
- [08:05] Zoomed out; sticky now typed: "this is linked list / your brain love it / simple , tiny" (cursor after "tiny").
- [08:06-08:07] Back to PDF Preview (page 73, Lists). Sticky final: "this is linked list / your brain love it / simple , tiny".
- [08:08-08:09] Clicks 2.9 Hash Tables in sidebar; PDF page 83: end of 2.8 Trees (parse tree for `mid = (low + high) / 2;`, Exercises 2-11..2-13), then heading "2.9. Hash Tables".
- [08:09-08:10] Page 84: Hash Tables text and figure ("symtab[NHASH]: hash chains:" with name/value boxes), then code for `Nameval` struct and `symtab[NHASH]`.
- [08:11-08:13] Page 85: `lookup` hash lookup/insert code; page 86: text on array size and `hash` function (MULTIPLIER = 31), start of its code.
- [08:14] Mission Control again; PDF window page 86; Firefox window with linked-list video; sticky unchanged.
- [08:15] Firefox tab 3 "Hash Table, for dummies like me" (youtube.com/watch?v=61XUoywg970&t=4s), his own video (2 likes). Video frame: vim `test.c` with a minimal hash table; a yellow sticky inside the video reads "this is very minimal hashtable with key is int".
- [08:16] Switches to tab 4 "Hayyoth Lab Forest Fire Simulat..." (Gemini chat gemini.google.com/app/94c12f3b2b7c65a7) showing the C `main` and "How to Run It".
- [08:18-08:19] Reloads that Gemini tab ("Waiting for gemini.google.com..."); blank Gemini with "Ask Gemini".
- [08:20] Chat loads: user prompt "# Hayyoth Lab / > We stole Zeus's lightning, trapped it in s... to think." and reply "Here is a complete Forest Fire simulation written for Hayyoth Lab, usin... types, and APIs described in the README." with "The Forest Fire Model": Empty (0), Tree (1), Fire (2). He starts typing in the prompt box: "has".
- [08:22-08:23] Clicks "New chat"; gemini.google.com/app shows "Where should we start?".
- [08:26] Prompt sent: "hash table in C"; Gemini thinking.
- [08:32] Gemini answer streams in. The cyan sticky (and webcam) are dragged to the right, partly off-screen; text unchanged. URL .../app/885dcfa126d97883.
- [08:33-08:34] "## Complete C Implementation Example"; code block begins. Webcam window title "Human" visible.
- [08:35-08:43] Scrolls up and down through the long Gemini code (see Code section): hash, ht_create, ht_insert, ht_search, ht_free, main example usage, "Key Considerations in C". Tab title "Implementing Hash Tables in C".
- [08:44] Clicks "+" for a new tab.
- [08:46] New tab, address bar "hayyoth.org/" with suggestions (Hayyoth - YouTube, Canva items, Reddit, codeberg hayyoth/hayyoth-lab "A small and fast C framework for building, running, and obser...", Google Translate, "The Dynamic Array. Conjured in C - YouTube").
- [08:47] Types "hash table wi" in address bar (suggestion: Hash table - Wikipedia).
- [08:49] Google search "hash table wiki" loading.
- [08:50] Results: Wikipedia "Hash table"; "Can't generate an AI overview right now. Try again later."; Simple English Wikipedia; Wikibooks "Data Structures/Hash Tables". Hovers first result.
- [08:51-08:58] English Wikipedia "Hash table": History/Overview ("An associative array stores a set of (key, value) pairs..."), Load factor; then scrolls through Collision resolution (separate chaining, pseudo-code, John Smith/Lisa Smith/Sam Doe/Sandra Dee/Ted Baker diagram), Open addressing (probing), Robin Hood hashing, Dynamic resizing, Implementations list, See also; scrolls back up to Hash function / Collision resolution at 08:58. Cyan sticky unchanged throughout.

## Texts on screen

### Yellow sticky (collapsed title strip), 07:38-08:58
```
How we learn better than AI?...
```

### Cyan sticky, version A, 07:38-07:39
```
it work

AI love complex

your brain not
```

### Cyan sticky, version B, 07:43-08:04 (typed live)
```
Hard to understand 
```

### Cyan sticky, version C (final), 08:05-08:58
```
this is linked list

your brain love it

simple , tiny
```

### Yellow sticky inside his own "Hash Table, for dummies like me" video (08:15)
```
this is very minimal hashtable with key is int
```

### Gemini prompts
- 08:20 existing chat prompt (partly hidden): `# Hayyoth Lab` / `> We stole Zeus's lightning, trapped it in s⟨?⟩ ... to think.` ?? (middle hidden behind sticky)
- 08:20 typed in prompt box (abandoned): `has`
- 08:26 sent: `hash table in C`

### Gemini answer prose (08:32-08:34 and 08:41)
```
Implementing a hash table in C requires building it from scratch, as C does not ha⟨?⟩
hash map data structures like Python ( dict ) or JavaScript ( Object ).

The standard approach for handling hash tables in C is using Separate Chaining, where e⟨?⟩
bucket points to a linked list to resolve collisions (when two keys hash to the same index).

## Complete C Implementation Example
Below is a complete, working implementation of a string-keyed hash table storing ⟨?⟩ values.
```
Later heading "## Key Considerations in C" (08:41), partly occluded by sticky:
```
Memory Management: Because C does not practice automatic garbage coll⟨?⟩ must explicitly call strdup() (or malloc()) when storing string keys, and ren⟨?⟩ free() both the keys and nodes inside your ht_free function to prevent ⟨?⟩
Hash Collisions: The example above utilizes Separate Chaining via linked lists⟨?⟩ keys evaluate to the same bucket index, they are chained together. Alternativ⟨?⟩ strategies include Open Addressing (like linear probing or quadratic probing⟨?⟩
```

### PDF text (vdoc.pdf, pp. 73, 83-86), key visible passages
- p.73: "This takes O(n) time and there's no way to improve that bound in general. Even if the list is sorted, we need to walk along the list to get to a particular element. Binary search does not apply to lists. To print the elements of a list, we can write a function to walk the list and print each element; to compute the length of a list, we can write a function to walk the list and increment a counter; and so on. An alternative is to write one function, apply, that walks a list and calls another function for each list element. We can make apply more flexible by providing it with an argument to be passed each time it calls the function. So apply has three arguments: the list, a function to be applied to each element of the list, and an argument for that function:" (next page: "The second argument of apply is a pointer to a function that takes two arguments and returns void. The standard but awkward syntax, void (*fn)(Nameval*, void*)")
- p.83: "mid = (low + high) / 2; can be represented by the parse tree shown in the figure below. To evaluate the tree, do a post-order traversal and perform the appropriate operation at each node." / "We'll take a longer look at parse trees in Chapter 9." / Exercise 2-11 "Compare the performance of lookup and nrlookup. How expensive is recursion compared to iteration?" / Exercise 2-12 "Use in-order traversal to create a sort routine. What time complexity does it have? Under what conditions might it behave poorly? How does its performance compare to our quicksort and a library version?" / Exercise 2-13 "Devise and implement a set of tests for verifying that the tree routines are correct." / "2.9. Hash Tables: Hash tables are one of the great inventions of computer science. They combine arrays, lists, and some mathematics to create an efficient structure for storing and retrieving dynamic data. The typical application is a symbol table, which associates some value (the data) with each member of a dynamic set of strings (the keys)."
- p.84: "The idea is to pass the key through a hash function to generate a hash value that will be evenly distributed through a modest-sized integer range. ... In practice, the hash function is pre-defined and an appropriate size of array is allocated, often at compile time. ... a hash table of n items is an array of lists whose average length is n/(array size). Retrieving an item is an O(1) operation provided we pick a good hash function and the lists don't grow too long. Because a hash table is an array of lists, the element type is the same as for a list:"
- p.85-86: "The list techniques we discussed in Section 2.7 can be used to maintain the individual hash chains. ... Here is the code for a hash table lookup/insert routine. ... How big should the array be? ... We must now decide what the hash function, hash, should calculate. The function must be deterministic and should be fast and distribute the data uniformly throughout the array. One of the most common hashing algorithms for strings builds a hash value by adding each byte of the string to a multiple of the hash so far. ... Empirically, the values 31 and 37 have proven to be good choices for the multiplier in a hash function for ASCII strings."

### Wikipedia "Hash table" (08:50-08:58), excerpts
```
Overview: An associative array stores a set of (key, value) pairs and allows insertion, deletion, and lookup (search), with the constraint of unique keys. In the hash table implementation of associative arrays, an array A of length m is partially filled ... A key x is hashed using a hash function h to compute an index location A[h(x)] in the hash table, ...

Load factor: The efficiency of a hash table depends on the load factor (alpha), defined as the ratio of the num⟨?⟩ ... load factor (alpha) = n/m, where n is the number of key-value pairs in the hash table. m is the number of buckets ...

Separate chaining: In separate chaining, the process involves building a linked list of key-value pairs for each search array index. The collided items are chained together through a single linked list, which can be traversed during lookup of a specific key. Collision resolution through chaining with linked lists is a common method of implementation of hash tables. ...

Open addressing is another collision resolution technique in which every entry record is stored in the bucket array itself, and the hash resolution is performed through probing. ... Well-known probe sequences include: Linear probing / Quadratic probing / Double hashing.

Dynamic resizing: Repeated insertions cause the number of entries in a hash table to grow ... Resizing by moving all entries / Alternatives to all-at-once rehashing
Implementations list: JavaScript object/Map, C++11 unordered_map, Go map, Java HashSet/HashMap/LinkedHashSet, Python dict, Ruby Hash, Rust HashMap/HashSet, .NET HashSet/Dictionary
See also: Bloom filter, Consistent hashing, Distributed hash table, Extendible hashing, Hash array mapped trie, Lazy deletion, Pearson hashing, PhotoDNA, Rabin-Karp string search algorithm, ...
```

## Code & shell

### vim main.c in his "Creation of Chain - Linked List" video (07:45-08:14); status line `"main.c" [New] 18L, 222B written`
```
 1  #include <stdio.h>
 2
 3  typedef struct Node {
 4    int val;
 5    struct Node *next;
 6  } Node;
 7
 8  int main(void) {
 9    Node n1 = {.val=10};
10    Node n2 = {.val=20};
11    Node n3 = {.val=30};
12
13    n1.next = &n2;
14    n2.next = &n3;
15
16
17    return 0;
18  }
```
Right terminal of that video (Chez Scheme):
```
lab $ # lisp list
lab $ chez
Chez Scheme Version 10.4.1
Copyright 1984-2026 Cis⟨?⟩
> (define my-list '(10 2⟨?⟩
> my-list
(10 20 30)
> (append my-list '(40 5⟨?⟩
(10 20 30 40 50)
>
```

### vim test.c in "Hash Table, for dummies like me" video (08:15); status `"main.c" 27L, 342B written   24,4   All`
```
 1  #include <stdio.h>
 2
 3  typedef struct Node {
 4    int value;
 5    int key;
 6    struct Node *next;
 7  } Node;
 8
 9  int main(void) {
10    Node arr[5] = {0};
11
12    //arr[2] = 22;
13    arr[2] = (Node) {
14      .value = 22,
15      .key = 2,
16      .next = NULL
17    };
18
19    //arr[7 % 5] = 77;
20    arr[2].next = &(Node) {
21      .value = 77,
22      .key = 7,
23      .next = NULL
24    };
25
26    return 0;
27  }
```

### PDF code, p.73 (`lookup`, `apply`)
```
/* lookup: sequential search for name in listp */
Nameval *lookup(Nameval *listp, char *name)
{
    for ( ; listp != NULL; listp = listp->next)
        if (strcmp(name, listp->name) == 0)
            return listp;
    return NULL;    /* no match */
}

/* apply: execute fn for each element of listp */
void apply(Nameval *listp,
        void (*fn)(Nameval*, void*), void *arg)
{
    for ( ; listp != NULL; listp = listp->next)
        (*fn)(listp, arg);  /* call the function */
}
```

### PDF code, pp.84-86
```
typedef struct Nameval Nameval;
struct Nameval {
    char    *name;
    int     value;
    Nameval *next;      /* in chain */
};

Nameval *symtab[NHASH]; /* a symbol table */

/* lookup: find name in symtab, with optional create */
Nameval* lookup(char *name, int create, int value)
{
    int h;
    Nameval *sym;

    h = hash(name);
    for (sym = symtab[h]; sym != NULL; sym = sym->next)
        if (strcmp(name, sym->name) == 0)
            return sym;
    if (create) {
        sym = (Nameval *) emalloc(sizeof(Nameval));
        sym->name = name; /* assumed allocated elsewhere */
        sym->value = value;
        sym->next = symtab[h];
        symtab[h] = sym;
    }
    return sym;
}

if (lookup("name") == NULL)
    additem(newitem("name", value));

enum { MULTIPLIER = 31 };
/* hash: compute hash value of string */
unsigned int hash(char *str)
{
    unsigned int h;
    unsigned char *p;

    h = 0;
    for (p = (unsigned char *) str; *p !=   ?? (cut off)
```

### Gemini forest-fire code (07:44, 08:16-08:19; tail of earlier chat, "C")
```
  return (HrdView){.r = 0, .g = 0, .b = 0};        // Black for empty space
}

int main(void) {
  HcellConf cellConf = {.width = 250, .height = 250, .cell⟨?⟩ = sizeof(Cell)};
  HrdConf rdConf = {.fps = 15, .scale = 3};

  if (!hSetup(cellConf, rdConf))
    return 1;

  hDataInit(init);

  while (hRun()) {
    hCell(rule);
    hRender(view);
  }

  return 0;
}
```

### Gemini answer to "hash table in C" (08:33-08:44)
```
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define TABLE_SIZE 100

// 1. Define the node for separate chaining
typedef struct HashNode {
    char *key;
    int value;
    struct HashNode *next;
} HashNode;

// 2. Define the hash table structure
typedef struct {
    HashNode *buckets[TABLE_SIZE];
} HashTable;

// 3. Simple hash function (djb2 algorithm variant)
unsigned int hash(const char *key) {
    unsigned int hash_val = 0;
    while (*key) {
        hash_val = (hash_val * 31) + *key++;
    }
    return hash_val % TABLE_SIZE;
}

// 4. Create and initialize the hash table
HashTable* ht_create() {
    HashTable *table = (HashTable*)malloc(sizeof(HashTable));
    for (int i = 0; i < TABLE_SIZE; i++) {
        table->buckets[i] = NULL;
    }
    return table;
}

// 5. Insert or update a key-value pair
void ht_insert(HashTable *table, const char *key, int value) {
    unsigned int idx = hash(key);
    HashNode *node = table->buckets[idx];

    // Check if key already exists, update value if found
    while (node != NULL) {
        if (strcmp(node->key, key) == 0) {
            node->value = value;
            return;
        }
        node = node->next;
    }

    // Otherwise, create a new node and insert at the head of the bucket l⟨?⟩
    HashNode *new_node = (HashNode*)malloc(sizeof(HashNode));
    new_node->key = strdup(key);
    new_node->value = value;
    new_node->next = table->buckets[idx];
    table->buckets[idx] = new_node;
}

// 6. Search for a value by key
int ht_search(HashTable *table, const char *key, int *found) {
    unsigned int idx = hash(key);
    HashNode *node = table->buckets[idx];

    while (node != NULL) {
        if (strcmp(node->key, key) == 0) {
            *found = 1;
            return node->value;
        }
        node = node->next;
    }

    *found = 0;
    return 0; // Return dummy value if not found
}

// 7. Free all dynamically allocated memory
void ht_free(HashTable *table) {
    for (int i = 0; i < TABLE_SIZE; i++) {
        HashNode *node = table->buckets[i];
        while (node != NULL) {
            HashNode *temp = node;
            ?? (rest of loop body not seen: advance node, free key/temp)
        }
    }
    ?? (...)
    }
    free(table); // Free the table container
}

// --- Example Usage ---
int main() {
    HashTable *table = ht_create();

    // Insert key-value pairs
    ht_insert(table, "apple", 50);
    ht_insert(table, "banana", 120);
    ht_insert(table, "cherry", 75);

    // Search for keys
    int found = 0;
    int val = ht_search(table, "banana", &found);
    if (found) {
        printf("Found 'banana': %d\n", val);
    } else {
        printf("'banana' not found.\n");
    }

    // Update an existing key
    ht_insert(table, "apple", 99);
    val = ht_search(table, "apple", &found);
    if (found) {
        printf("Updated 'apple': %d\n", val);
    }

    // Clean up memory
    ht_free(table);
    return 0;
}
```

### Wikipedia pseudo-code (08:55)
```
Chained-Hash-Insert(T, x)
  insert x at the head of linked list T[h(k)]

Chained-Hash-Search(T, k)
  search for an element with key k in linked list T[h(k)]

Chained-Hash-Delete(T, x)
  delete x from the linked list T[h(k)]
```

## Observations
- This segment is a staged comparison. He first shows the same topic, linked lists and hash tables, in a thick textbook (Kernighan/Pike, 733-page PDF) with function-pointer abstraction (`apply`, `void (*fn)(Nameval*, void*)`) and writes on the cyan sticky "it work / AI love complex / your brain not", then "Hard to understand". He then cuts to his own scratch videos: an 18-line C linked list (two typedef lines, three nodes, two pointer assignments) and a minimal int-key hashtable, and rewrites the sticky as "this is linked list / your brain love it / simple , tiny".
- He then returns to the textbook's hash table chapter, then shows the AI/encyclopedia route: Gemini asked "hash table in C" (a long answer with malloc, strdup, free, separate chaining, 7 numbered parts and a "Key Considerations" lecture) and the English Wikipedia "Hash table" article (collision resolution, open addressing, Robin Hood hashing, resizing, load factor). He scrolls long distances through both without comment on screen, letting the sheer volume carry the point, while the sticky keeps saying "simple, tiny".
- He uses the sticky as the only typed argument; it is edited live. Everything else is demonstration: switching windows with Mission Control, tabs of his own videos, a Gemini chat that already contains his "Hayyoth Lab" README prompt (Zeus's lightning, trapped it in s... to think) and a forest-fire simulation generated against it, then a fresh chat.
- Philosophy visible here: complexity (abstraction, completeness, optimization, generality) is what AI and textbooks produce and what a human brain rejects; the brain loves the smallest concrete thing you can build and see from zero (a Node struct and two pointers). His channel Hayyoth ("Conjured in C", "for dummies like me"), C with vim in a plain terminal, and a Scheme list beside the C list are offered as the alternative path.
- Privacy: no e-mail or account name appeared in this segment; the Google page profile icon is shown without a name.
