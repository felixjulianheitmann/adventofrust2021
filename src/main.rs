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

    let mut risk = 0;
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
                risk += this + 1;
            }
        }
    }

    util::write_output(risk);
}
