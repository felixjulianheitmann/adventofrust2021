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
On the other hand, it might be wise to try a different strategy: let the giant squid win.

You aren't sure how many bingo boards a giant squid could play at once, so rather than waste time counting its arms, the safe thing to do is to figure out which board will win last and choose that one. That way, no matter which boards it picks, it will win for sure.

In the above example, the second board is the last to win, which happens after 13 is eventually called and its middle column is completely marked. If you were to keep playing until this point, the second board would have a sum of unmarked numbers equal to 148 for a final score of 148 * 13 = 1924.

Figure out which board will win last. Once it wins, what would its final score be?