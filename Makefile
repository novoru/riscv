CFLAGS=-Wall -g -O0 -lreadline
SRCS=$(wildcard *.c ./sh/*.c ./test/*.c ./vm/*.c)
OBJS=$(SRCS:.c=.o)
SUBDIRS=./src/test/testcase

run:
	cargo run

test:
	$(MAKE) -C $(SUBDIRS)
	cargo test

clean:
	$(MAKE) clean -C $(SUBDIRS)
	cargo clean
