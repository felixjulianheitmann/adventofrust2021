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
It seems like the individual flashes aren't bright enough to navigate. However, you might have a better option: the flashes seem to be synchronizing!

In the example above, the first time all octopuses flash simultaneously is step 195:

```
After step 193:
5877777777
8877777777
7777777777
7777777777
7777777777
7777777777
7777777777
7777777777
7777777777
7777777777

After step 194:
6988888888
9988888888
8888888888
8888888888
8888888888
8888888888
8888888888
8888888888
8888888888
8888888888

After step 195:
0000000000
0000000000
0000000000
0000000000
0000000000
0000000000
0000000000
0000000000
0000000000
0000000000
```

If you can calculate the exact moments when the octopuses will all flash simultaneously, you should be able to navigate through the cavern. What is the first step during which all octopuses flash?