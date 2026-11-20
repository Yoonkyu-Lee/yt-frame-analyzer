CC = cc
CFLAGS = -std=c11 -Wall -Wextra -Wpedantic -Werror -g

.PHONY: all test sanitize clean

all: main

main: main.c list.c list.h
	$(CC) $(CFLAGS) main.c list.c -o $@

test_list: test_list.c list.c list.h
	$(CC) $(CFLAGS) test_list.c list.c -o $@

test: test_list
	./test_list

sanitize: test_list.c list.c list.h
	$(CC) $(CFLAGS) -fsanitize=address,undefined -fno-omit-frame-pointer test_list.c list.c -o test_list_sanitize
	./test_list_sanitize

clean:
	rm -f main test_list test_list_sanitize
