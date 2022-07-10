pub fn read_input() -> String {
    std::fs::read_to_string("input.txt").expect("Something went wrong reading the input...")
}

pub fn split_lines<T>(text: String) -> Vec<T>
where T: std::str::FromStr,
<T as std::str::FromStr>::Err: std::fmt::Debug
{
    let lines = text.split('\n');
    lines.map(|l| l.parse().unwrap()).collect()
}
    
pub fn split_lines_str(text: String) -> Vec<String> {
    text.split('\n').map(|l| String::from(l)).collect()
}

pub fn split_at<T>(token: char, text: &str) -> Vec<&str> {
    text.split(token).collect()
}

pub fn write_output<T>(out: T) where T: std::fmt::Display {
    std::fs::write("output.txt", format!("The result is: {}", out.to_string())).expect("Writing to output.txt failed...")
}
