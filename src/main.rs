pub mod util;

fn main() {
    let mut dumbos = util::split_lines_str(util::read_input())
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
                    _ => 0,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let safe_inc = |v: &mut Vec<Vec<i32>>, r: usize, r_inc: i32, c: usize, c_inc: i32| {
        if (r == 0 && r_inc < 0) || (r == v.len() && r_inc > 0) {
            return;
        }
        if (c == 0 && c_inc < 0) || (r == v.len() && c_inc > 0) {
            return;
        }
        if let Some(col) = v.get_mut(((r as i32) + r_inc) as usize) {
            if let Some(x) = col.get_mut(((c as i32) + c_inc) as usize) {
                *x += 1;
            }
        }
    };

    let mut iter = 1;
    loop {
        let mut flashed_map: Vec<Vec<bool>> = Vec::new();
        flashed_map.resize(dumbos.len(), Vec::new());
        flashed_map
            .iter_mut()
            .for_each(|v| v.resize(dumbos[0].len(), false));

        // Step 1) Increase all energy levels
        dumbos.iter_mut().flatten().for_each(|x| *x += 1);

        // Step 2) Flash while not all flashable dumbos have flashed yet
        while dumbos
            .iter()
            .flatten()
            .zip(flashed_map.iter().flatten())
            .any(|(energy, has_flashed)| *energy > 9 && !has_flashed)
        {
            for r in 0..dumbos.len() {
                for c in 0..dumbos[0].len() {
                    if dumbos[r][c] > 9 && !flashed_map[r][c] {
                        safe_inc(&mut dumbos, r, -1, c, -1);
                        safe_inc(&mut dumbos, r, -1, c, 0);
                        safe_inc(&mut dumbos, r, -1, c, 1);
                        safe_inc(&mut dumbos, r, 0, c, -1);
                        safe_inc(&mut dumbos, r, 0, c, 1);
                        safe_inc(&mut dumbos, r, 1, c, -1);
                        safe_inc(&mut dumbos, r, 1, c, 0);
                        safe_inc(&mut dumbos, r, 1, c, 1);
                        flashed_map[r][c] = true;
                    }
                }
            }
        }

        dumbos
            .iter_mut()
            .flatten()
            .zip(flashed_map.iter().flatten())
            .for_each(|(energy, has_flashed)| {
                if *has_flashed {
                    *energy = 0;
                }
            });

        if flashed_map.iter().flatten().into_iter().all(|x| *x) {
            break;
        }
        iter += 1;
    }
    util::write_output(iter);
}
