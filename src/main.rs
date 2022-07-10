pub mod util;

fn main() {
    let lines: Vec<i32> = util::split_lines(util::read_input());

    let mut n_inc = 0;
    let mut prev: i32 = lines[..2].iter().sum();

    for window in lines[1..].windows(3) {
        let sum = window.iter().sum();
        if prev < sum {
            n_inc += 1;
        }
        prev = sum;
    }

    util::write_output(n_inc);

}
