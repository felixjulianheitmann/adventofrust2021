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
    let field_check = |height: i32, checked_yet: &mut bool, basin_size: &mut i32| -> bool {
        let mut still_basin = false;
        if height < 9 && !*checked_yet {
            *basin_size += 1;
            still_basin = true;
        }
        *checked_yet = true;
        still_basin
    };

    // Check neighbors
    match heights.get(((*x as i32) - 1) as usize) {
        Some(col) => {
            if field_check(
                *col.get(*y).unwrap(),
                checked_map.get_mut(x - 1).unwrap().get_mut(*y).unwrap(),
                basin_size,
            ) {
                calc_basin_size(&(x - 1), y, heights, checked_map, basin_size);
            }
        }
        None => (),
    };

    match heights.get(x + 1) {
        Some(col) => {
            if field_check(
                *col.get(*y).unwrap(),
                checked_map.get_mut(x + 1).unwrap().get_mut(*y).unwrap(),
                basin_size,
            ) {
                calc_basin_size(&(x + 1), y, heights, checked_map, basin_size);
            }
        }
        None => (),
    };

    match heights.get(*x).unwrap().get(((*y as i32) - 1) as usize) {
        Some(val) => {
            if field_check(
                *val,
                checked_map.get_mut(*x).unwrap().get_mut(y - 1).unwrap(),
                basin_size,
            ) {
                calc_basin_size(x, &(y - 1), heights, checked_map, basin_size);
            }
        }
        None => (),
    };
    match heights.get(*x).unwrap().get(y + 1) {
        Some(val) => {
            if field_check(
                *val,
                checked_map.get_mut(*x).unwrap().get_mut(y + 1).unwrap(),
                basin_size,
            ) {
                calc_basin_size(x, &(y + 1), heights, checked_map, basin_size);
            }
        }
        None => (),
    };

    *basin_size
}
