CC = cc
CFLAGS = -std=c11 -Wall -Wextra -Wpedantic -Werror -O2

.PHONY: all test sanitize clean

all: main

main: main.c dynarray.c dynarray.h
	$(CC) $(CFLAGS) main.c dynarray.c -o $@

test_dynarray: test_dynarray.c dynarray.c dynarray.h
	$(CC) $(CFLAGS) test_dynarray.c dynarray.c -o $@

test: test_dynarray
	./test_dynarray

sanitize: test_dynarray.c dynarray.c dynarray.h
	$(CC) -std=c11 -Wall -Wextra -Wpedantic -Werror -g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer test_dynarray.c dynarray.c -o test_dynarray_sanitize
	./test_dynarray_sanitize

clean:
	rm -f main test_dynarray test_dynarray_sanitize
