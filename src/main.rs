pub mod util;

use std::collections::hash_map::HashMap;

struct Cave {
    name: String,
    is_large: bool,
    neighbors: Vec<String>,
}

fn main() {
    let vertices = util::split_lines_str(util::read_input())
        .iter()
        .map(|s| util::split_at::<String>("-", s.to_string()))
        .collect::<Vec<Vec<_>>>();

    let mut caves: HashMap<String, Cave> = HashMap::new();

    let create_if_new = |caves: &mut HashMap<String, Cave>, key: &str| {
        if !caves.contains_key(key) {
            caves.insert(
                key.to_string(),
                Cave {
                    name: key.to_string(),
                    is_large: (key.to_uppercase() == key),
                    neighbors: Vec::new(),
                },
            );
        }
    };

    for vertice in vertices {
        create_if_new(&mut caves, &vertice[0]);
        create_if_new(&mut caves, &vertice[1]);
        caves
            .entry(vertice[0].to_string())
            .and_modify(|c| c.neighbors.push(vertice[1].to_string()));
        caves
            .entry(vertice[1].to_string())
            .and_modify(|c| c.neighbors.push(vertice[0].to_string()));
    }

    let start = &caves["start"];
    util::write_output(path_finder(&start, &caves, HashMap::new(), false));
}

fn path_finder(
    p: &Cave,
    caves: &HashMap<String, Cave>,
    mut visited: HashMap<String, i32>,
    mut double_visit: bool,
) -> i32 {
    let end = &caves["end"];
    let mut n_paths = 0;
    if let Some(times_seen) = visited.get_mut(&p.name) {
        *times_seen += 1;
        if !p.is_large {
            double_visit = true;
        }
    } else {
        visited.insert(p.name.clone(), 1);
    }

    for n in p
        .neighbors
        .iter()
        .filter(|c| {
            let x = caves.get(*c).unwrap();
            let seen_already = match visited.get(*c) {
                Some(b) => *b,
                None => 0,
            };
            x.is_large
                || seen_already == 0
                || (seen_already == 1 && !double_visit && x.name != "start" && x.name != "end")
        })
        .map(|s| caves.get(s).unwrap())
    {
        if n.name == end.name {
            n_paths += 1;
        } else {
            n_paths += path_finder(n, caves, visited.clone(), double_visit);
        }
    }
    return n_paths;
}
