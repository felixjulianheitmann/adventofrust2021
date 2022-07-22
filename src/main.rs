pub mod util;

use std::collections::HashSet;

fn main() {
    let mut vec = util::split_at::<String>("\n\n", util::read_input());
    let mut template = vec.remove(0);
    let rules = vec.remove(0);

    let rules = util::split_lines_str(rules.to_string())
        .into_iter()
        .map(|l| util::split_at::<String>(" -> ", l))
        .collect::<Vec<_>>();

    const N_STEPS: i32 = 10;

    for _ in 0..N_STEPS {
        let mut new_template = "".to_string();
        for pair in template.as_bytes().windows(2) {
            let rule = rules.iter().find(|v| v[0].as_bytes() == pair).unwrap();
            new_template.push(pair[0] as char);
            new_template.push(rule[1].chars().next().unwrap());
        }
        new_template.push(template.chars().last().unwrap());
        template = new_template;
    }

    let letters = template.chars().collect::<HashSet<_>>();
    let mut occurrences: Vec<usize> = Vec::new();
    for letter in letters {
        occurrences.push(template.chars().filter(|c| *c == letter).count());
    }
    occurrences.sort();

    util::write_output(occurrences.last().unwrap() - occurrences.iter().next().unwrap());
}
