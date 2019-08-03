SHELL:=/bin/bash
run:
	printf 'foo1\tfoo2\tfoo3\nbar1\tbar2\tbar3\n' | cargo run
	cargo run data.tsv

test:
	cargo test
