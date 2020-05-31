run:
	cargo run

test:
	cargo test --quiet

clean:
	cargo clean

bench:
	cargo bench --lib --benches --quiet

.PHONY: run test clean bench