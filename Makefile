SHELL:=/bin/bash
rustup:
	curl https://sh.rustup.rs -sSf | sh

update:
	rustup update
	rustup self update

run:
	printf 'foo1\tfoo2\tfoo3\nbar1\tbar2\tbar3\n' | cargo run
	cargo run data.tsv

test:
	cargo test
