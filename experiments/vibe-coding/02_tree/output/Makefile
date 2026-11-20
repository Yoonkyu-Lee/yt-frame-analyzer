CC = cc
CFLAGS = -std=c11 -O2 -Wall -Wextra -Wpedantic

.PHONY: all test clean
all: ctree

ctree: main.c
	$(CC) $(CFLAGS) main.c -o $@

test: ctree
	python3 tests.py ./ctree

clean:
	rm -f ctree
