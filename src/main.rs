pub mod util;

use std::collections::{HashMap, HashSet};

fn main() {
    let mut vec = util::split_at::<String>("\n\n", util::read_input());
    let template = vec.remove(0);
    let last_letter = template.chars().last().unwrap() as u8;
    let rules = vec.remove(0);

    let rules = util::split_lines_str(rules.to_string())
        .into_iter()
        .map(|l| util::split_at::<String>(" -> ", l))
        .collect::<Vec<_>>();

    const N_STEPS: i32 = 40;

    // Transform template into HashMap for better scaling
    let mut template = template
        .as_bytes()
        .windows(2)
        .map(|pair| ((pair[0], pair[1]), 0))
        .collect::<HashMap<_, usize>>();

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

    let mut occurrences = template
        .iter()
        .map(|((l, _), n)| (n, l))
        .collect::<Vec<_>>();

    // Alternative ->
    //  Collect letters as singles in Hashmap with their occurences.
    //   - Add if the letters is already present
    //   - Create new if letter is not in there yet
    //  Update letter which is 'last_letter' with +=1
    //  Collect as vector of usize
    //  Sort vector

    util::write_output(occurrences.last().unwrap().0 - occurrences.iter().next().unwrap().0);
}
