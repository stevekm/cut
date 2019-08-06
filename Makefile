SHELL:=/bin/bash
rustup:
	curl https://sh.rustup.rs -sSf | sh

update:
	rustup update
	rustup self update

build:
	cargo build

run:
	printf 'foo1\tfoo2\tfoo3\nbar1\tbar2\tbar3\n' | cargo run -- -f 1
	cargo run data.tsv -f 1
	cargo run data.tsv -f 1,2
	cargo run data.tsv -f 1,2,5-8
	cargo run data.tsv -f 2,6-

test:
	cargo test
