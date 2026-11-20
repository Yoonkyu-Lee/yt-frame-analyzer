CC = cc
CFLAGS = -std=c11 -Wall -Wextra -Wpedantic -Werror -O2

.PHONY: all test clean
all: main

main: main.c hash_table.c hash_table.h
	$(CC) $(CPPFLAGS) $(CFLAGS) main.c hash_table.c $(LDFLAGS) $(LDLIBS) -o $@

test_hash_table: test_hash_table.c hash_table.c hash_table.h
	$(CC) $(CPPFLAGS) $(CFLAGS) test_hash_table.c hash_table.c $(LDFLAGS) $(LDLIBS) -o $@

test: test_hash_table
	./test_hash_table

clean:
	rm -f main test_hash_table test_hash_table_sanitize
