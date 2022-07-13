pub mod util;

fn sort(lhs: usize, rhs: usize) -> (usize, usize) {
    if lhs < rhs {
        return (lhs, rhs);
    } else {
        return (rhs, lhs);
    }
}

fn main() {
    let vents: Vec<Vec<_>> = 
        util::split_lines_str(util::read_input()).iter()
            .map(|l| util::split_at::<String>(" -> ", l.to_string())
                .iter()
                .map(|p| util::split_at::<usize>(",", p.to_string())).collect()).collect();

    // initialze ocean floor
    let mut geo: Vec<Vec<i32>> = Vec::new();
    geo.resize(1000, Vec::new());
    geo.iter_mut().for_each(|r| r.resize(1000, 0));

    for vent in vents {
        if vent[0][0] != vent[1][0] && vent[0][1] != vent[1][1] {
            continue;
        }
        let row = sort(vent[0][0], vent[1][0]);
        let col = sort(vent[0][1], vent[1][1]);
        for r in row.0..=row.1 {
            for c in col.0..=col.1 {
                geo[r][c] += 1;
            }
        }
    }

    util::write_output(geo.iter().flatten().map(|x| *x).filter(|x| *x > 1).count());

}
