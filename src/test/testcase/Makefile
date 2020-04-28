LD = riscv64-linux-gnu-ld
AS = riscv64-linux-gnu-as
OBJCOPY = riscv64-linux-gnu-objcopy

CFLAGS  = -nostdlib -O0
ASFLAGS = -fno-pic -march=rv64i -mabi=lp64
LDFLAGS = -e start --oformat=binary
OBJFLAGS = -O binary --only-section=.text
TARGETS = add addi slti sltiu andi ori xori slli srli srai\
		  lui auipc slt sltu and or xor sll srl sub sra nop\
		  beq bne blt bltu bge bgeu jal jalr\
		  lsb lsh lsw

SRCS=$(wildcard *.s)
OBJS=$(SRCS:.s=.o)

all: $(TARGETS)

$(TARGETS): $(OBJS) $(SRCS)
	$(AS) $(ASFLAGS) $(basename $@).s -o $@
	$(OBJCOPY) $(OBJFLAGS) $@.o $(basename $@)
	#$(LD) $(LDFLAGS) $@.o -o $(basename $@)

clean:
	rm $(OBJS) $(basename $(OBJS))