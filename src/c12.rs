use std::{io::{self, BufRead}, collections::HashSet};

// ugh, this was unnecessary and i misread the problem
// fn max_one_small_cave(path_so_far: Vec<&str>) -> bool {
//     let mut count = 0;
//     // println!("trying path {:?}", path_so_far.clone());
//     for location in path_so_far {
//         if location != "start" && location != "end" && location.chars().all(|c| c.is_ascii_lowercase()) {
//             if count == 1 {
//                 // println!("failing at {}", location);
//                 return false;
//             } else {
//                 // println!("adding at {}", location);
//                 count += 1;
//             }
//         }
//     }
//     return true;
// }

fn get_paths<'a>(path_so_far: Vec<&'a str>, visited_caves: HashSet<String>, edges: &'a Vec<(String, String)>) -> Vec<Vec<&'a str>> {
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
            if &edge.0 == current && (!visited_caves.contains(&edge.1) || edge.1.chars().all(|c| c.is_ascii_uppercase())) {
                let mut new_path = path_so_far.clone();
                new_path.push(&edge.1);
                let mut new_visited = visited_caves.clone();
                new_visited.insert(edge.1.to_string()); 
                // println!("trying path {:?}", new_path);
                paths.append(&mut get_paths(new_path, new_visited, edges));
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

    let paths = get_paths(vec!["start"], HashSet::from(["start".to_string()]), &edges);
    // println!("Paths found: {:?}", paths);
    println!("Total paths: {}", paths.len());
    // let foo: &Vec<&str> = paths.iter().filter(|p| max_one_small_cave(p.to_vec())).collect();
    // println!("Paths with at most one small cave: {}", paths.iter().filter(|p| max_one_small_cave(p.to_vec())).count());
}
