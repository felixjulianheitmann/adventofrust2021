pub mod util;

fn main() {
    let n_days = 80;

    let init = util::split_at::<usize>(",", util::read_input());
    let mut population = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for fish in init {
        population[fish] += 1;
    }

    for _ in 0..n_days {
        let mut population_new = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        for (idx, age) in population.iter().enumerate() {
            match idx {
                0 => {
                    population_new[6] += *age;
                    population_new[8] += *age;
                }
                _ => population_new[idx - 1] += *age,
            }
        }
        population = population_new;
    }

    util::write_output(population.iter().sum::<usize>());
}
