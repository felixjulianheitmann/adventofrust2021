pub mod util;

fn main() {
    let lines: Vec<i32> = util::split_lines(util::read_input());

    let mut n_inc = 0;
    let mut prev = lines[0];

    for depth in &lines[1..] {
        if prev < *depth {
            n_inc += 1;
        }
        prev = *depth;
    }

    util::write_output(n_inc);

}
