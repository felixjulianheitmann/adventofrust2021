pub mod util;

use std::collections::HashMap;

fn main() {
    let mut vec = util::split_at::<String>("\n\n", util::read_input());
    let template_str = vec.remove(0);
    let last_letter = template_str.chars().last().unwrap() as u8;
    let rules = vec.remove(0);

    let rules = util::split_lines_str(rules.to_string())
        .into_iter()
        .map(|l| util::split_at::<String>(" -> ", l))
        .collect::<Vec<_>>();

    const N_STEPS: i32 = 40;

    // Transform template into HashMap for better scaling
    let mut template: HashMap<(u8, u8), usize> = HashMap::new();
    for pair in template_str.as_bytes().windows(2) {
        let pair = (pair[0], pair[1]);
        if let Some(p) = template.get_mut(&pair) {
            *p += 1;
        } else {
            template.insert(pair, 1);
        }
    }

    for _ in 0..N_STEPS {
        let mut new_template: HashMap<(u8, u8), usize> = HashMap::new();
        for (pair, n) in template {
            let rule = rules
                .iter()
                .find(|v| {
                    (
                        v[0].chars().next().unwrap() as u8,
                        v[0].chars().last().unwrap() as u8,
                    ) == pair
                })
                .unwrap();

            let new_pairs = [
                (pair.0, rule[1].chars().next().unwrap() as u8),
                (rule[1].chars().next().unwrap() as u8, pair.1),
            ];

            for pair in new_pairs {
                if let Some(new_n) = new_template.get_mut(&pair) {
                    *new_n += n;
                } else {
                    new_template.insert(pair, n);
                }
            }
        }

        template = new_template;
    }

    let mut occurrences: HashMap<u8, usize> = HashMap::new();
    for (pair, n) in template {
        if let Some(x) = occurrences.get_mut(&pair.0) {
            *x += n;
        } else {
            occurrences.insert(pair.0, n);
        }
    }

    // Update because last letter has not yet been accounted for
    *occurrences.get_mut(&last_letter).unwrap() += 1;

    let mut occurrences = occurrences
        .into_iter()
        .map(|(_, n)| n)
        .collect::<Vec<usize>>();
    occurrences.sort();

    util::write_output(occurrences.last().unwrap() - occurrences.iter().next().unwrap());
}
