use std::iter::Peekable;

pub mod util;

fn main() {
    let lines = util::split_lines_str(util::read_input());
    let mut scores: Vec<usize> = Vec::new();
    for line in lines {
        let mut it = line.chars().peekable();
        let mut score_acc = 0;
        while !it.peek().is_none() {
            if let Some((score_l, it_rest)) = check_block(it.next().unwrap(), it) {
                it = it_rest;
                score_acc += score_l;
            } else {
                break;
            }
        }
        if score_acc > 0 {
            scores.push(score_acc);
        }
    }

    scores.sort();
    util::write_output(scores[(scores.len() - 1) / 2]);
}

fn check_block<'a>(
    ot: char,
    mut it: Peekable<std::str::Chars<'a>>,
) -> Option<(usize, Peekable<std::str::Chars<'a>>)> {
    let mut score = 0;
    loop {
        if let Some(_) = it.peek() {
            let nt = it.next().unwrap();
            // Is block opening token?
            if nt == '(' || nt == '[' || nt == '{' || nt == '<' {
                if let Some((score_tmp, it_rest)) = check_block(nt, it) {
                    score += score_tmp;
                    it = it_rest;
                } else {
                    return None;
                }
            }
            // Is block closing token?
            else {
                match ot {
                    '(' => match nt {
                        ')' => return Some((0, it)),
                        _ => return None,
                    },
                    '[' => match nt {
                        ']' => return Some((0, it)),
                        _ => return None,
                    },
                    '{' => match nt {
                        '}' => return Some((0, it)),
                        _ => return None,
                    },
                    '<' => match nt {
                        '>' => return Some((0, it)),
                        _ => return None,
                    },
                    _ => return None,
                }
            }
        // No token left
        } else {
            return Some((score * 5 + symbol2score(ot), it));
        }
    }
}

fn symbol2score(s: char) -> usize {
    match s {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}
