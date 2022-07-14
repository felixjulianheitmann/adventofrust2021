pub mod util;

fn main() {
    let input = util::split_lines_str(util::read_input())
        .iter()
        .map(|l| {
            util::split_at::<String>(" | ", l.to_string())
                .iter()
                .map(|s| util::split_at::<String>(" ", s.to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut sum = 0;
    for line in input {
        let code = &line[0];
        let value = &line[1];

        let one = code.iter().find(|s| s.len() == 2).unwrap();
        let seven = code.iter().find(|s| s.len() == 3).unwrap();
        let four = code.iter().find(|s| s.len() == 4).unwrap();
        let eight = code.iter().find(|s| s.len() == 7).unwrap();

        let a = difference(seven, one);
        let cf = overlap(seven, one);
        let bd = difference(four, one);
        let aeg = difference(eight, four);
        let eg = difference(&aeg, &a);

        // find zero
        let acefg = a.clone() + &cf + &eg;
        let zero = code
            .iter()
            .find(|s| s.len() == 6 && acefg.chars().all(|c| s.contains(c)))
            .unwrap();
        let b = overlap(&bd, &zero);
        let d = difference(&bd, &zero);

        // find five
        let abd = a.clone() + &bd;
        let five = code
            .iter()
            .find(|s| s.len() == 5 && abd.chars().all(|c| s.contains(c)))
            .unwrap();
        let c = difference(&cf, &five);
        let f = overlap(&cf, &five);
        let e = difference(&eg, &five);
        let g = overlap(&eg, &five);

        // decode value
        sum += digits2num(
            value
                .iter()
                .map(|s| {
                    code2digit(&decode(
                        &s,
                        &a.chars().nth(0).unwrap(),
                        &b.chars().nth(0).unwrap(),
                        &c.chars().nth(0).unwrap(),
                        &d.chars().nth(0).unwrap(),
                        &e.chars().nth(0).unwrap(),
                        &f.chars().nth(0).unwrap(),
                        &g.chars().nth(0).unwrap(),
                    ))
                })
                .collect::<Vec<_>>(),
        );
    }

    util::write_output(sum);
}

fn decode<'a>(
    code: &'a str,
    a: &char,
    b: &char,
    c: &char,
    d: &char,
    e: &char,
    f: &char,
    g: &char,
) -> String {
    code.chars()
        .map(|x| {
            if x == *a {
                return 'a';
            }
            if x == *b {
                return 'b';
            }
            if x == *c {
                return 'c';
            }
            if x == *d {
                return 'd';
            }
            if x == *e {
                return 'e';
            }
            if x == *f {
                return 'f';
            }
            if x == *g {
                return 'g';
            }
            panic!("It has to be one of the valig letters");
        })
        .collect::<String>()
}

fn code2digit(code: &str) -> i32 {
    let mut code = code.chars().collect::<Vec<_>>();
    code.sort();
    let code: String = code.into_iter().collect();
    match code.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => -1,
    }
}

fn digits2num(digits: Vec<i32>) -> i32 {
    digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3]
}

fn difference<'a>(base: &'a str, cmp: &'a str) -> String {
    base.chars().filter(|c| !cmp.contains(*c)).collect()
}

fn overlap<'a>(base: &'a str, cmp: &'a str) -> String {
    base.chars().filter(|c| cmp.contains(*c)).collect()
}
