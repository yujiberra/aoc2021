use std::{io::{self, BufRead}, collections::HashMap};

type Point = (u8, u8);

#[derive(Clone, Debug)]
struct Path {
    nodes: Vec<Point>,
    cost: i32,
}

// fn manhattan(p_1: Point, p_2: Point) -> i16 {
//     return (p_1.0 as i8 - p_2.0 as i8).abs() as i16 +
//         (p_1.1 as i8 - p_2.1 as i8) as i16;
// }

fn neighbors(point: Point, height: u8, width: u8) -> Vec<Point> {
    let mut points = Vec::new();
    if point.0 > 0 {
        points.push((point.0 - 1, point.1));
    }
    if point.0 < height - 1 {
        points.push((point.0 + 1, point.1));
    }
    if point.1 > 0 {
        points.push((point.0, point.1 - 1))
    }
    if point.1 < width - 1 {
        points.push((point.0, point.1 + 1));
    }
    return points;
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut grid: Vec<Vec<u8>> = Vec::new();
    
    for line in lines {
        grid.push(line
            .unwrap() .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect());
    }

    // println!("{:?}", grid);

    let mut best_paths: HashMap<Point, Path> = HashMap::new();

    let height = grid.len() as u8;
    let width = grid[0].len() as u8;
    let destination = (height - 1, width - 1);

    best_paths.insert((0, 0), Path {
        nodes: vec![(0, 0)],
        cost: 0,
    });

    while !best_paths.contains_key(&destination) {
        let mut candidates: Vec<(Point, Path)> = Vec::new();
        for point in best_paths.keys() {
            for neighbor in neighbors(*point, height, width) {
                if !best_paths.contains_key(&neighbor) {
                    let path_so_far = best_paths.get(&point).unwrap();
                    let mut new_path: Path = path_so_far.clone();
                    new_path.nodes.push(neighbor);
                    new_path.cost = path_so_far.cost + grid[neighbor.0 as usize][neighbor.1 as usize] as i32;
                    candidates.push((neighbor, new_path));
                }
            }
        }
        candidates.sort_by(|a, b| {
            let distance_order = a.1.cost.cmp(&b.1.cost);
            // if distance_order != Ordering::Equal {
                return distance_order;
            // }
            //  else {
            //     return manhattan(b.0, destination).cmp(&manhattan(a.0, destination));
            // }
        });
        // println!("candidates: {:?}", candidates);
        // println!("best_paths.keys: {:?}, {}", best_paths.keys(), best_paths.keys().len());
        // println!("destination: {:?}", destination);
        let min_dist = candidates[0].1.cost;
        for candidate in candidates {
            if candidate.1.cost == min_dist {
                best_paths.insert(candidate.0, candidate.1);
            } else {
                break;
            }
        }
    }
    println!("Answer: {:?}", best_paths.get(&destination).unwrap());
    println!("Answer: {}", best_paths.get(&destination).unwrap().cost);

}
