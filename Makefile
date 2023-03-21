
build:
	cargo build --release

run:
	cargo run -- another test.txt -vv

run-verbose:
	cargo run -- another test.txt -vvv

test:
	cargo test