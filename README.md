# Advent Of Rust 2021

This repo tries to implement all the puzzles from the [Advent of code 2021](https://adventofcode.com/).

Each branch solves one puzzle and has a second branch for the second half of each puzzle. All branches are named `puzzle_<no>_<1/2>` where the `<no>` is the puzzle number and `<1|2>` tells which half of the puzzle is solved by this branch.

## Building

Of course, an installed version of rust is needed. With cargo installed, all puzzles should build using
```bash
cargo build     # to build
cargo run       # to run
```

The inputs and outputs of each puzzle are stored in `input.txt` and `output.txt`.

## Puzzle description

--- Part Two ---
Suppose the lanternfish live forever and have unlimited food and space. Would they take over the entire ocean?

After 256 days in the example above, there would be a total of 26984457539 lanternfish!

How many lanternfish would there be after 256 days?