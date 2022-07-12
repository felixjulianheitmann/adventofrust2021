pub mod util;

fn filter_value(mut lines: Vec<String>, filter_common: bool) -> i32 {
    let drop_a;
    let drop_b;
    match filter_common {
        true => {
            drop_a = '1';
            drop_b = '0';
        }
        false => {
            drop_a = '0';
            drop_b = '1';
        }
    };

    for idx in 0..lines[0].len() {
        let mut bit = 0;
        for line in &lines {
            match line.chars().nth(idx).unwrap() {
                '1' => bit += 1,
                '0' => bit -= 1,
                _ => continue,
            }
        }

        if bit >= 0 {
            // More 1 than 0
            lines.retain(|s| s.chars().nth(idx).unwrap() == drop_a);
        } else {
            // More 0 than 1
            lines.retain(|s| s.chars().nth(idx).unwrap() == drop_b);
        }

        if lines.len() <= 1 {
            break;
        }
    }

    let value: Vec<_> = lines[0]
        .chars()
        .map(|c| match c {
            '1' => 1,
            '0' => 0,
            _ => 0,
        })
        .collect();
    value
        .iter()
        .map(|v| if *v > 0 { 1 } else { 0 })
        .fold(0, |acc, b| (acc << 1) + b)
}

fn main() {
    let lines = util::split_lines_str(util::read_input());

    let co2 = filter_value(lines.clone(), true);
    let o2 = filter_value(lines.clone(), false);

    util::write_output(co2 * o2);
}
