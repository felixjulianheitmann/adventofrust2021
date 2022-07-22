pub mod util;

use std::collections::HashSet;

fn main() {
    type DotSet = HashSet<(i32, i32)>;

    let input = util::split_at::<String>("\n\n", util::read_input());
    let mut dots = util::split_lines_str(input[0].to_string())
        .iter()
        .map(|l| {
            let mut l = l.split(',');
            (
                l.next().unwrap().parse().unwrap(),
                l.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<DotSet>();

    let folds = util::split_lines_str(input[1].to_string())
        .into_iter()
        .map(|l| {
            let mut l = l.strip_prefix("fold along ").unwrap().split('=');
            (
                l.next().unwrap().to_string(),
                l.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(String, i32)>>();

    // Start the folding
    for (axis, fold) in folds {
        let mut new_dots: DotSet = HashSet::new();

        for (x, y) in dots {
            match axis.as_str() {
                "x" => {
                    if x < fold {
                        new_dots.insert((2 * fold - x, y));
                    } else {
                        new_dots.insert((x, y));
                    }
                }
                "y" => {
                    if y > fold {
                        new_dots.insert((x, 2 * fold - y));
                    } else {
                        new_dots.insert((x, y));
                    }
                }
                _ => (),
            };
        }

        dots = new_dots;
        break; // Only first iteration is desired
    }

    util::write_output(dots.len());
}
