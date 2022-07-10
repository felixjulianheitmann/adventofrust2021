
pub fn read_input() -> String {
    std::fs::read_to_string("input.txt").expect("Something went wrong reading the input...")
}

pub fn split_lines(text: &str) -> Vec<&str> {
    text.split('\n').collect()
}

pub fn split_at(token: char, text: &str) -> Vec<&str> {
    text.split(token).collect()
}