use std::collections::HashMap;

pub mod util;

struct Path {
    cost: i32,
    visited_nodes: Vec<(usize, usize)>,
}

fn main() {
    let risks = util::split_lines_str(util::read_input())
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    _ => panic!("Only integer inputs 0-9 are allowed"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut paths: HashMap<(usize, usize), Path> = HashMap::new();

    let mut start = paths.insert(
        (0, 0),
        Path {
            cost: risks[0][0],
            visited_nodes: Vec::new(),
        },
    );

    let x_min = 0;
    let y_min = 0;
    let x_max = risks.len();
    let y_max = risks[0].len();

    loop {
        let shortespaths
            .iter()
            .min_by(|(_, lhs), (_, rhs)| lhs.cost.cmp(&rhs.cost)).unwrap();
        for x in 0..paths.len() {
            for y in 0..paths[0].len() {
                if paths[x][y].evaluated {
                    let update_neighbor = |paths: &mut Vec<Vec<Path>>,
                                           risks: &Vec<Vec<i32>>,
                                           x: usize,
                                           x_inc: i32,
                                           y: usize,
                                           y_inc: i32| {
                        if (x == x_min && x_inc < 0)
                            || (y == y_min && y_inc < 0)
                            || (y == y_max - 1 && y_inc > 0)
                            || (x == x_max - 1 && x_inc > 0)
                        {
                            // Out of grid
                            return;
                        }

                        let new_x = ((x as i32) + x_inc) as usize;
                        let new_y = ((y as i32) + y_inc) as usize;
                        let neighbor = &mut paths[new_x][new_y];
                        let neighbor_risk = risks[new_x][new_y];
                        let this = paths[x][y];

                        if this.visited_nodes.contains(&(new_x, new_y)) {
                            // This neighbor has been visited on this path already
                            return;
                        }

                        if neighbor.evaluated && (this.cost + neighbor_risk > neighbor.cost) {
                            // The neighbor has a better path to it's cell
                            return;
                        }

                        // update neighbor
                        neighbor.visited_nodes = this.visited_nodes.clone();
                        neighbor.visited_nodes.push((x, y));
                    };
                }
            }
        }
    }

    util::write_output(0);
}
