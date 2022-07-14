pub mod util;

fn sum_series(n: i32) -> i32 {
    (1..=n).fold(0, |acc, x| acc + x)
}

fn main() {
    let crabs: Vec<i32> = util::split_at(",", util::read_input());
    let max_pos = crabs.iter().max().unwrap();
    let mut costs: Vec<i32> = Vec::new();
    costs.resize((*max_pos + 1) as usize, 0);

    for pos in 0..=*crabs.iter().max().unwrap() {
        costs[pos as usize] = crabs
            .iter()
            .fold(0, |acc, crab_pos| acc + sum_series((pos - crab_pos).abs()));
    }

    util::write_output(costs.iter().min().unwrap());
}
