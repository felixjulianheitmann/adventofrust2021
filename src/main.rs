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

    paths.insert(
        (0, 0),
        Path {
            cost: risks[0][0],
            visited_nodes: Vec::new(),
        },
    );

    let x_max = risks.len();
    let y_max = risks[0].len();

    loop {
        let ((x, y), current) = paths
            .iter()
            .min_by(|(_, lhs), (_, rhs)| lhs.cost.cmp(&rhs.cost))
            .unwrap();

        if *x == x_max - 1 && *y == y_max - 1 {
            break;
        }

        update_neighbor(&mut paths, &risks, current, x_max, y_max, *x, 1, *y, 0);
        update_neighbor(&mut paths, &risks, current, x_max, y_max, *x, -1, *y, 0);
        update_neighbor(&mut paths, &risks, current, x_max, y_max, *x, 0, *y, 1);
        update_neighbor(&mut paths, &risks, current, x_max, y_max, *x, 0, *y, -1);
    }

    let ((x, y), _) = paths
        .iter()
        .min_by(|(_, lhs), (_, rhs)| lhs.cost.cmp(&rhs.cost))
        .unwrap();

    // Temporary output
    util::write_output(x * y);
}

fn update_neighbor(
    paths: &mut HashMap<(usize, usize), Path>,
    risks: &Vec<Vec<i32>>,
    current: &Path,
    x_max: usize,
    y_max: usize,
    x: usize,
    x_inc: i32,
    y: usize,
    y_inc: i32,
) {
    if (x == 0 && x_inc < 0)
        || (y == 0 && y_inc < 0)
        || (y == y_max - 1 && y_inc > 0)
        || (x == x_max - 1 && x_inc > 0)
    {
        // Out of grid
        return;
    }

    let new_x = ((x as i32) + x_inc) as usize;
    let new_y = ((y as i32) + y_inc) as usize;
    let neighbor = match paths.get_mut(&(new_x, new_y)) {
        Some(n) => n,
        None => {
            let mut p = Path {
                cost: current.cost + risks[new_x][new_y],
                visited_nodes: current.visited_nodes.clone(),
            };
            p.visited_nodes.push((x, y));
            paths.insert((new_x, new_y), p);
            return;
        }
    };

    let neighbor_risk = risks[new_x][new_y];

    if current.visited_nodes.contains(&(new_x, new_y)) {
        // This neighbor has been visited on this path already
        return;
    }

    if current.cost + neighbor_risk > neighbor.cost {
        // The neighbor has a better path to it's cell
        return;
    }

    // update neighbor
    neighbor.visited_nodes = current.visited_nodes.clone();
    neighbor.visited_nodes.push((x, y));
}
