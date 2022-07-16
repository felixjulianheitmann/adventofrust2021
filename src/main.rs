use std::iter::Peekable;

pub mod util;

fn main() {
    let lines = util::split_lines_str(util::read_input());
    let mut score = 0;
    for line in lines {
        let mut it = line.chars().peekable();
        let (score_l, _) = check_block(it.next().unwrap(), it);
        score += score_l;
    }

    util::write_output(score);
}

fn check_block<'a>(
    ot: char,
    mut it: Peekable<std::str::Chars<'a>>,
) -> (i32, Peekable<std::str::Chars<'a>>) {
    let mut score = 0;
    loop {
        if let Some(_) = it.peek() {
            let nt = it.next().unwrap();
            // Is block opening token?
            if nt == '(' || nt == '[' || nt == '{' || nt == '<' {
                let score_tmp;
                (score_tmp, it) = check_block(nt, it);
                score += score_tmp;
            }
            // Is block closing token?
            else {
                match ot {
                    '(' => match nt {
                        ')' => return (score, it),
                        ']' | '}' | '>' => return (score + symbol2score(nt), it),
                        _ => return (score, it),
                    },
                    '[' => match nt {
                        ']' => return (score, it),
                        ')' | '}' | '>' => return (score + symbol2score(nt), it),
                        _ => return (score, it),
                    },
                    '{' => match nt {
                        '}' => return (score, it),
                        ')' | ']' | '>' => return (score + symbol2score(nt), it),
                        _ => return (score, it),
                    },
                    '<' => match nt {
                        '>' => return (score, it),
                        ')' | ']' | '}' => return (score + symbol2score(nt), it),
                        _ => return (score, it),
                    },
                    _ => return (score, it),
                }
            }
        // No token left
        } else {
            return (score, it);
            // return (score + symbol2score(ot), it);
        }
    }
}

fn symbol2score(s: char) -> i32 {
    match s {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        '(' => 3,
        '[' => 57,
        '{' => 1197,
        '<' => 25137,
        _ => 0,
    }
}
