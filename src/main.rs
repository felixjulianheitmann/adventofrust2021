pub mod util;

fn main() {
    let mut pos_h = 0;
    let mut pos_v = 0;
    let mut aim = 0;

    for line in &util::split_lines_str(util::read_input()) {
        let cmd = util::split_at(' ', line);
        match cmd[0] {
            "forward"=> {
                let v = cmd[1].parse::<i32>().unwrap();
                pos_h += v;
                pos_v += v * aim;
            },
            "up"=> aim -= cmd[1].parse::<i32>().unwrap(),
            "down"=> aim += cmd[1].parse::<i32>().unwrap(),
            _ => continue,
        }
    }
    util::write_output(pos_h * pos_v);
}
