# `rust-sort`
A simple program to make a file of random float values, and arrange them.

## How to install
- git clone the repository in you prefered directory.
- run `cargo run`. [View How to use]
- To arrange, use `./arrange` [view How to use]

## How to use
- After cloning the repo[view How to install], use by:
	## Creating Random Numbers
	- There are two arguments:
	- **file** -> Provide the file name in which to write the random numbers.[prefereably end with `.csv`]
	- **number** -> Provide the amount of numbers to be created.
	- Eg:
	``` sh
	cargo run 100000_nums.csv 100000
	```
	- Format:
	- `cargo run <file name> <number>`

	## Arranging them:
	- There is only ONE necessary argument.
	- **file** -> Provide the file to be sorted.
	- Running this is a bit different.
	- Eg:
	``` sh
	./ascend 100000_nums.csv
	```
	- It shall be written in `docs/ascending.csv` and `docs/descending.csv`.

## Necessary crates
Only one: `rand` [version = 0.8.4]
It will install when you run `cargo run`, but mentioned.

## Enjoy!
