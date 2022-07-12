pub mod util;

fn main() {
    let lines = util::split_lines_str(util::read_input());
    let mut bit_vecs: Vec<i32> = Vec::new();
    bit_vecs.resize(lines[0].len(), 0);

    // count bits
    for line in &lines {
        for (idx, c) in line.chars().enumerate() {
            match c {
                '1' => bit_vecs[idx] += 1,
                '0' => bit_vecs[idx] -= 1,
                _ => continue,
            };
        }
    }

    // fold bit vector to integer
    let gamma = bit_vecs
        .iter()
        .map(|v| if *v > 0 { 1 } else { 0 })
        .fold(0, |acc, b| (acc << 1) + b);

    let epsilon = bit_vecs
        .iter()
        .map(|v| if *v > 0 { 0 } else { 1 })
        .fold(0, |acc, b| (acc << 1) + b);

    util::write_output(gamma * epsilon);
}
