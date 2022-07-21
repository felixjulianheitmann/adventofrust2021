pub mod util;

use std::{collections::HashSet, io::BufRead};

fn main() {
    let input = util::split_at::<String>("\n\n", util::read_input());
    let mut dots = util::split_lines_str(input[0])
        .iter()
        .map(|l| {
            let l = l.split(',');
            (
                l.next().unwrap().parse().unwrap(),
                l.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<HashSet<(i32, i32)>>();

    let folds = util::split_lines_str(input[1])
        .iter()
        .map(|l| {
            let l = l.strip_prefix("fold along ").unwrap().split('=');
            (l.next().unwrap(), l.next().unwrap().parse().unwrap())
        })
        .collect::<Vec<(&str, i32)>>();

    // Start the folding
    for (axis, fold) in folds {
        match axis {
            "y" => {
                for (x, y) in dots.into_iter().filter(|(x, y)| *x < fold) {
                    dots.insert((2 * fold - x, y));
                    dots.remove(&(x, y));
                }
            }
            "x" => {
                for (x, y) in dots.into_iter().filter(|(_, y)| *y > fold) {
                    dots.insert((x, 2 * fold - y));
                    dots.remove(&(x, y));
                }
            }
        };
    }

    util::write_output(input);
}
