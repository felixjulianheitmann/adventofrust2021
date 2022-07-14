pub mod util;

fn main() {
    let input = util::split_lines_str(util::read_input())
        .iter()
        .map(|l| {
            util::split_at::<String>(" | ", l.to_string())
                .iter()
                .map(|s| util::split_at::<String>(" ", s.to_string()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    util::write_output::<usize>(
        input
            .iter()
            .map(|x| {
                x[1].iter()
                    .filter(|c| c.len() == 2 || c.len() == 3 || c.len() == 4 || c.len() == 7)
                    .count()
            })
            .sum(),
    );
}
