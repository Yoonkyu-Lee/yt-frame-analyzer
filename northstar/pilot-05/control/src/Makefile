CC = cc
CFLAGS = -std=c11 -O2 -Wall -Wextra -Wpedantic
LDLIBS = -lm
PROGRAMS = touch cat mkdir mv rm find

.PHONY: all clean
all: $(PROGRAMS)

$(PROGRAMS): %: %.o common.o

%.o: %.c common.h

clean:
	$(RM) $(PROGRAMS) *.o
