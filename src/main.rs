pub mod util;

fn main() {
    let lines = util::split_lines_str(util::read_input());
    let mut score = 0;
    for line in lines {
        let (mut score_l, mut it_rest) = check_block(&line.chars());
        score += score_l;
        while let Some(_) = it_rest.peekable().peek() {
            (score_l, it_rest) = check_block(it_rest);
            score += score_l;
        }
    }

    util::write_output(score);
}

fn check_block<'a>(it: &'a std::str::Chars<'a>) -> (i32, &'a std::str::Chars<'a>) {

    let ot = it.next().unwrap();

    loop {

        let nt = match it.next() {
            Some(t) => t,
            None => return (symbol2score(ot), it),
        };
        match token {
            '(' => match nt {
                ')' => (0, &line[1..]),
                ']' | '}' | '>' => (symbol2score(nt), &line[1..]),
                _ => {
                    let (score, rest) = check_block(nt, &line[1..]);
                    match rest.chars().next() {
                        Some(t) => 
                    }
    
                }
            },
            '[' => match nt {
                ']' => 0,
                ')' | '}' | '>' => symbol2score(nt),
                _ => {
                    let (score, rest) = check_block(nt, &line[1..]);
    
                }
            },
            '{' => match nt {
                '}' => 0,
                ')' | ']' | '>' => symbol2score(nt),
                _ => {
                    let (score, rest) = check_block(nt, &line[1..]);
    
                }
            },
            '<' => match nt {
                '>' => 0,
                ')' | ']' | '}' => symbol2score(nt),
                _ => {
                    let (score, rest) = check_block(nt, &line[1..]);
    
                }
            },
            _ => 0,
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
