pub mod util;

fn sum_series(cache: &mut Vec<usize>, n: usize) -> usize {
    match (n) < cache.len() {
        true => cache[n],
        false => {
            // cache.resize(n, 0);
            for x in cache.len()..=n {
                cache.push(cache.last().unwrap() + x);
            }
            *cache.last().unwrap()
        }
    }
}

fn main() {
    let crabs: Vec<i32> = util::split_at(",", util::read_input());
    let max_pos = crabs.iter().max().unwrap();
    let mut costs: Vec<usize> = Vec::new();
    costs.resize((*max_pos + 1) as usize, 0);

    let mut cache = vec![0, 1, 3, 6];

    for pos in 0..=*crabs.iter().max().unwrap() {
        costs[pos as usize] = crabs.iter().fold(0, |acc, crab_pos| {
            acc + sum_series(&mut cache, (pos - crab_pos).abs() as usize)
        });
    }

    println!("Cache size: {:?}", cache.len());
    util::write_output(costs.iter().min().unwrap());
}
