OBJCOPY = riscv64-unknown-elf-objcopy

OBJFLAGS = -O binary --only-section=.text.init
DESTDIR = ./rvtests
SRCDIR = ./riscv-tests/isa/

RVTESTS = rv64ui-p-add rv64ui-p-addi rv64ui-p-addw 
PATH = 

all: $(addprefix $(SRCDIR), $(RVTESTS))
	$(OBJCOPY) $(addprefix $(SRCDIR), $(RVTESTS)) $(RVTESTS)

clean:
	rm $(OBJS) $(basename $(OBJS))