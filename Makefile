SUBDIRS=./src/test/testcase
RVTESTS=./src/test/riscv-tests

run:
	cargo run

test:
	$(MAKE) -C $(SUBDIRS)
	cargo test

clean:
	$(MAKE) clean -C $(SUBDIRS)
	cargo clean
