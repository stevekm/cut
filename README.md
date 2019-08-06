[![Build Status](https://travis-ci.org/stevekm/cut.svg?branch=master)](https://travis-ci.org/stevekm/cut)
# cut

GNU cut implemented in Rust

# Installation

## Build From Source

First, clone this repository:

```
git clone https://github.com/stevekm/cut.git
cd cut
```

If `rustup` is not already installed, install it with `curl https://sh.rustup.rs -sSf | sh`.

Build the program with `cargo build`.

# Usage

The `Makefile` includes some example methods to run the program; `make run`.

By default the program reads from `stdin`.

```
$ printf 'foo1\tfoo2\tfoo3\nbar1\tbar2\tbar3\n' | target/debug/cut -f2
foo2
bar2
```

A range of fields can be passed, similar to GNU `cut`.

```
$ target/debug/cut -f1-3,6-9 data.tsv
1	2	3	6	7	8	9
11	22	33	66	77	88	99

$ target/debug/cut -f6- data.tsv
6	7	8	9
66	77	88	99
```

Individual fields can be specified out of order as well to change output order

```
$ target/debug/cut -f 9,1-2 data.tsv
9	1	2
99	11	22

$ target/debug/cut -f 6,5,4 data.tsv
6	5	4
66	55	44
```
