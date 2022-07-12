pub mod util;

fn main() {
    let lines = util::split_lines_str(util::read_input());
    let draws = util::split_at::<&i32>(',', &lines[0]);

    for draw in draws {
        for mut board in lines[1..].chunks(6) {
            board = &board[1..];
            // use vector of vectors for representation
        }
    }

}
