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
            let mut decision_value = x;
            let mut new_tup = (2 * fold - x, y);
            match axis.as_str() {
                "y" => {
                    decision_value = y;
                    new_tup = (x, 2 * fold - y);
                }
                _ => (), // "x" is covered as default above
            };
            if decision_value > fold {
                new_dots.insert(new_tup);
            } else {
                new_dots.insert((x, y));
            }
        }

        dots = new_dots;
    }

    let max_x = dots.iter().map(|(x, _)| x).max().unwrap();
    let max_y = dots.iter().map(|(_, y)| y).max().unwrap();

    let mut result_str = "\n".to_string();
    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if let Some(_) = dots.get(&(x, y)) {
                result_str += format!("#").as_str();
            } else {
                result_str += format!("-").as_str();
            }
        }
        result_str += "\n";
    }

    util::write_output(result_str);
}
