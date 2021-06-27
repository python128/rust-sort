# `rust-sort`
A simple program to make a file of random float values, and arrange them.

## How to install
- git clone the repository.
- run `cargo build --release`
- run `mv target/release/sorting /usr/local/bin/sorting`

## How to use
- There are two modes. `make` and `sort`.
- `make` - creates the random numbers' file.
- `sort` - sortes the given file.
- FORMAT - `cargo run <mode> <file-name> <amount-of-nums>*`
###### *is only necessary for mode1[make]

### `make`
- Example: `cargo run make rand-nums-100000.csv 100000`
- Explanation:
- `cargo run` - to build the binary, and run it.
- `make` - explained in how to use.
- `rand-nums-100000.csv` - File name in which to store the numbers.
- `100000` - Amount of numbers.
- FORMAT - `cargo run make <file-name> <amount-of-nums>`

### `sort`
- Example - `cargo run sort rand-nums-100000.csv`
- Explanation:
- `cargo run` - Builds binary and runs it.
- `sort` - mode2[explained in How to use]
- `rand-nums-100000.csv` - File to sort.
- FORMAT - `cargo run sort <file-to-sort>`
