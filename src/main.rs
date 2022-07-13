pub mod util;

fn has_won(board: &Vec<Vec<i32>>, draws: &Vec<i32>) -> (bool, i32) {
    if draws.len() < 5 {
        return (false, 0);
    }

    let mut won = false;
    let mut score = 0;

    for line in board {
        if line.iter().all(|i| draws.contains(i)) {
            won = true;
        }
    }

    if board.iter().any(|l| l.iter().all(|i| draws.contains(i))) {
        won = true;
    }

    for column in 0..board[0].len() {
        if board
            .iter()
            .flatten()
            .skip(column)
            .step_by(board[0].len())
            .all(|x| draws.contains(x))
        {
            won = true;
            break;
        }
    }

    if won {
        score = board
            .iter()
            .flatten()
            .filter(|x| !draws.contains(*x))
            .fold(0, |acc, x| acc + x);
    }

    score *= draws.last().unwrap();
    (won, score)
}

fn main() {
    let lines = util::split_lines_str(util::read_input());
    let draws = util::split_at::<i32>(",", lines[0].clone());

    let boards: Vec<Vec<Vec<i32>>> = lines[1..]
        .chunks(6)
        .map(|s| {
            s[1..]
                .iter()
                .map(|x| {
                    let mut x: Vec<_> = x.split(" ").collect();
                    x.retain(|s| *s != "");
                    x.iter().map(|s| s.parse().unwrap()).collect()
                })
                .collect()
        })
        .collect();

    let mut draws_yet: Vec<i32> = Vec::new();
    for draw in draws {
        draws_yet.push(draw);
        for board in &boards {
            let (won, score) = has_won(board, &draws_yet);
            if won {
                util::write_output(score);
            }
        }
    }
}
