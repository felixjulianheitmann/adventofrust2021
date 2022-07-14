pub mod util;

fn main() {
    let lines = util::split_lines_str(util::read_input());
}

fn check_block<'a>(open_token: char, line: &'a str) -> (i32, &'a str) {
    
    let mut token = open_token;
    loop {

        let nt = match line.chars().next() {
            Some(t) => t,
            None => return (symbol2score(open_token), line),
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
