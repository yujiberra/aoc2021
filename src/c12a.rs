use std::{io::{self, BufRead}, collections::HashSet};

fn get_paths<'a>(path_so_far: Vec<&'a str>, visited_caves: HashSet<String>, edges: &'a Vec<(String, String)>, used_double: bool) -> Vec<Vec<&'a str>> {
    let current = path_so_far[path_so_far.len() - 1];
    // println!("current = {}", current);
    if current == "end" {
        let foo = vec![path_so_far; 1]; 
        return foo;
    } else {
        let mut paths: Vec<Vec<&str>> = Vec::new();
        for edge in edges {
            // if &edge.0 == current && !visited_caves.contains(&edge.1) {
            // println!("checking destination {}, {}", edge.1, edge.1.chars().all(|c| c.is_ascii_uppercase()));
            let dest_is_big = edge.1.chars().all(|c| c.is_ascii_uppercase());
            let already_visited_dest = visited_caves.contains(&edge.1);
            if &edge.0 == current && (dest_is_big || !already_visited_dest || !used_double) {
                let used_double = used_double || (!dest_is_big && already_visited_dest);
                // println!("used_double = {}", used_double);
                let mut new_path = path_so_far.clone();
                new_path.push(&edge.1);
                let mut new_visited = visited_caves.clone();
                new_visited.insert(edge.1.to_string()); 
                // println!("trying path {:?}", new_path);
                paths.append(&mut get_paths(new_path, new_visited, edges, used_double));
            }
        }
        return paths;
    }
}

fn main() {
    let stdin = io::stdin();

    let mut edges: Vec<(String, String)> = Vec::new();

    for line in stdin.lock().lines() {
        let unwrapped = line.unwrap();
        let mut foo = unwrapped.split("-");
        let first = foo.next().unwrap();
        let second = foo.next().unwrap();
        if first != "end" && second != "start" {
            edges.push((String::from(first), String::from(second)));
        }
        if first != "start" && second != "end" {
            edges.push((String::from(second), String::from(first)));
        }
    }

    println!("edges {:?}", edges);

    let paths = get_paths(vec!["start"], HashSet::from(["start".to_string()]), &edges, false);
    // println!("Paths found: {:?}", paths);
    println!("Total paths: {}", paths.len());
    // let foo: &Vec<&str> = paths.iter().filter(|p| max_one_small_cave(p.to_vec())).collect();
    // println!("Paths with at most one small cave: {}", paths.iter().filter(|p| max_one_small_cave(p.to_vec())).count());
}
