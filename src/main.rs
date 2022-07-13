pub mod util;

fn sort(lhs: usize, rhs: usize) -> (usize, usize) {
    if lhs < rhs {
        return (lhs, rhs);
    } else {
        return (rhs, lhs);
    }
}

fn main() {
    let grid_size = 1000;

    let vents: Vec<Vec<_>> = 
        util::split_lines_str(util::read_input()).iter()
            .map(|l| util::split_at::<String>(" -> ", l.to_string())
                .iter()
                .map(|p| util::split_at::<usize>(",", p.to_string())).collect()).collect();

    // initialze ocean floor
    let mut geo: Vec<Vec<i32>> = Vec::new();
    geo.resize(grid_size, Vec::new());
    geo.iter_mut().for_each(|r| r.resize(grid_size, 0));

    // count vents
    for vent in vents {
        let (row, col) = ((vent[0][0], vent[1][0]), (vent[0][1], vent[1][1]));

        if row.0 != row.1 && col.0 != col.1 {
            // diagonal line
            let row_r = match row.0 > row.1 {
                true => (row.1..=row.0).rev().collect::<Vec<usize>>(),
                false => (row.0..=row.1).collect::<Vec<usize>>(),
            };
            let col_r = match col.0 > col.1 {
                true => (col.1..=col.0).rev().collect::<Vec<usize>>(),
                false => (col.0..=col.1).collect::<Vec<usize>>(),
            };

            for (r, c) in row_r.iter().zip(col_r.iter()) {
                geo[*r][*c] += 1;
            }
        } else {
            // horizontal/vertical line
            let row = sort(row.0, row.1);
            let col = sort(col.0, col.1);
            for r in row.0..=row.1 {
                for c in col.0..=col.1 {
                    geo[r][c] += 1;
                }
            }
        }
    }

    util::write_output(geo.iter().flatten().map(|x| *x).filter(|x| *x > 1).count());
}
