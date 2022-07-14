pub mod util;

fn main() {
    let lines = util::split_lines_str(util::read_input());
    let heights = lines
        .iter()
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
                    _ => panic!("Only numeric symbols allowed"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut basins: Vec<i32> = Vec::new();

    // find lowpoints
    for r in 0..heights.len() {
        for c in 0..heights[0].len() {
            let top = match heights.get(((r as i32) - 1) as usize) {
                Some(col) => *col.get(c).unwrap(),
                None => 10,
            };
            let bottom = match heights.get(r + 1) {
                Some(col) => *col.get(c).unwrap(),
                None => 10,
            };
            let left = match heights.get(r).unwrap().get(((c as i32) - 1) as usize) {
                Some(val) => *val,
                None => 10,
            };
            let right = match heights.get(r).unwrap().get(c + 1) {
                Some(val) => *val,
                None => 10,
            };
            let this = heights[r][c];
            if this < top && this < bottom && this < left && this < right {
                // Calculate basin size
                let mut checked_map: Vec<Vec<bool>> = Vec::new();
                checked_map.resize(heights.len(), Vec::new());
                checked_map
                    .iter_mut()
                    .for_each(|v| v.resize(heights[0].len(), false));
                checked_map[r][c] = true;
                let mut size = 1;

                basins.push(calc_basin_size(
                    &r,
                    &c,
                    &heights,
                    &mut checked_map,
                    &mut size,
                ));
            }
        }
    }

    // Calculate result
    let first = *basins.iter().max().unwrap();
    basins.remove(basins.iter().position(|x| *x == first).unwrap());
    let second = *basins.iter().max().unwrap();
    basins.remove(basins.iter().position(|x| *x == second).unwrap());
    let third = *basins.iter().max().unwrap();
    util::write_output(first * second * third);
}

fn calc_basin_size(
    x: &usize,
    y: &usize,
    heights: &Vec<Vec<i32>>,
    checked_map: &mut Vec<Vec<bool>>,
    basin_size: &mut i32,
) -> i32 {
    let field_check = |heights: &Vec<Vec<i32>>,
                       checked_map: &mut Vec<Vec<bool>>,
                       x: usize,
                       y: usize,
                       basin_size: &mut i32|
     -> bool {
        match heights.get(x) {
            Some(col) => match col.get(y) {
                Some(h) => {
                    let mut still_basin = false;
                    if *h < 9 && !checked_map[x][y] {
                        *basin_size += 1;
                        still_basin = true;
                    }
                    checked_map[x][y] = true;
                    return still_basin;
                }
                None => false,
            },
            None => false,
        }
    };

    // Check neighbors
    if *x > 0 {
        // Top
        if field_check(&heights, checked_map, x - 1, *y, basin_size) {
            calc_basin_size(&(x - 1), y, heights, checked_map, basin_size);
        }
    }

    // Bottom
    if field_check(&heights, checked_map, x + 1, *y, basin_size) {
        calc_basin_size(&(x + 1), y, heights, checked_map, basin_size);
    }

    if *y > 0 {
        // Left
        if field_check(&heights, checked_map, *x, y - 1, basin_size) {
            calc_basin_size(x, &(y - 1), heights, checked_map, basin_size);
        }
    }

    // Right
    if field_check(&heights, checked_map, *x, y + 1, basin_size) {
        calc_basin_size(x, &(y + 1), heights, checked_map, basin_size);
    }

    *basin_size
}
