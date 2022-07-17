pub mod util;

fn main() {
    let vertices = util::split_lines_str(util::read_input())
        .iter()
        .map(|s| util::split_at::<String>("-", *s))
        .collect::<Vec<Vec<_>>>();

    let caves: Vec<Cave> = Vec::new();

    for vertice in vertices {
        let start = &vertice[0];
        let end = &vertice[1];
        if 
    }

    util::write_output(0);
}

struct Cave<'a> {
    name: String,
    visited: bool,
    is_large: bool,
    neighbors: Vec<&'a Cave<'a>>,
}

impl Vec<Cave> {
    fn in_list(&self, c: &Cave) -> bool {
        &self.iter().any(|x| x.name == c.name)
    }

    fn add_vertice(&self, start: &str, end: &str) {
        match &self.iter().find(|x| &x.name == start) {
            Some(c) => *c.neighbors.push(Cave::new(start));
        }
    }
}
