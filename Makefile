
build:
	cargo build --release

run:
	cargo run -- another test.txt -vv

run-verbose:
	cargo run -- another test.txt -vvv

test:
	cargo test


dpush:
	docker buildx build -t caladreas/git-next-tag-rust:0.1.0-rc04 . --push