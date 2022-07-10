use util::split_lines_str;

pub mod util;

fn main() {
    let mut pos_h = 0;
    let mut pos_v = 0;

    for line in &util::split_lines_str(util::read_input()) {
        let cmd = util::split_at(' ', line);
        match cmd[0] {
            "forward"=> pos_h += cmd[1].parse::<i32>().unwrap(),
            "up"=> pos_v -= cmd[1].parse::<i32>().unwrap(),
            "down"=> pos_v += cmd[1].parse::<i32>().unwrap(),
            _ => continue,
        }
    }
    util::write_output(pos_h * pos_v);
}
