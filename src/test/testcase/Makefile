LD = riscv64-linux-gnu-ld
AS = riscv64-linux-gnu-as

CFLAGS  = -nostdlib -O0
ASFLAGS = -fno-pic -march=rv64i -mabi=lp64
LDFLAGS = -e start --oformat=binary
TARGETS = add addi slti

SRCS=$(wildcard *.s)
OBJS=$(SRCS:.s=.o)

all: $(TARGETS)

$(TARGETS): $(OBJS) $(SRCS)
	$(AS) $(ASFLAGS) $(basename $@).s -o $@
	$(LD) $(LDFLAGS) $@.o -T linker.ld -o $(basename $@)

clean:
	rm $(OBJS) $(basename $(OBJS))