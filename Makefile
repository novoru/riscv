SUBDIRS=./src/test/testcase

run:
	cargo run

test:
	$(MAKE) -C $(SUBDIRS)
	cargo test

clean:
	$(MAKE) clean -C $(SUBDIRS)
	cargo clean
