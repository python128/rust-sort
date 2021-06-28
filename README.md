# `rust-sort`
A simple program to make a file of random float values, and arrange them.

## How to use
- FORMAT - `cargo run <mode> <file-name> <amount-of-nums>*`
- There are two modes. `make` and `sort`.
- `make` - creates the random numbers' file.
- `sort` - sortes the given file.
###### *is only necessary for mode1[make]

### `make`
- FORMAT - `cargo run make <file-name> <amount-of-nums>`
- Example: `cargo run make rand-nums-100000.csv 100000`
- Explanation:
- `cargo run` - to build the binary, and run it.
- `make` - explained in how to use.
- `rand-nums-100000.csv` - File name in which to store the numbers.
- `100000` - Amount of numbers.

### `sort`
- FORMAT - `cargo run sort <file-to-sort>`
- Example - `cargo run sort rand-nums-100000.csv`
- Explanation:
- `cargo run` - Builds binary and runs it.
- `sort` - mode2[explained in How to use]

## How to install
- git clone the repository.
- run `cargo build --release`
- run `cargo install --path .` if you want to install it. `rand-nums-100000.csv` - File to sort.

## Necessities
- `rand` - will install, if not installed.
- Terminal(obviously).
- cargo & rust to be previously installed.
