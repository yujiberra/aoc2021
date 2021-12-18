use std::{io::{self, BufRead}, collections::HashSet};

fn basin(point: (u8, u8), found: &mut HashSet<(u8, u8)>, map: &Vec<Vec<u8>>, height: usize, width: usize) {
    for pp in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let i = point.0 as i16 + pp.0;
        let j = point.1 as i16 + pp.1;
        if i < 0 || i >= height as i16 {
            continue;
        }
        if j < 0 || j >= width as i16 {
            continue;
        }
        let candidate = (i as u8, j as u8);
        if map[i as usize][j as usize] < 9 && !found.contains(&candidate) {
            found.insert(candidate);
            basin(candidate, found, map, height, width);
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in stdin.lock().lines() {
        let unwrapped = line.unwrap();
        if unwrapped.len() == 0 {
            break;
        }
        let splits: Vec<u8> = unwrapped.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        map.push(splits);
    }

    let height = map.len();
    let width = map[0].len();

    let mut low_points: Vec<(u8, u8)> = Vec::new();

    for i in 0..height {
        for j in 0..width {
            // println!("now on {} at {},{}", map[i][j], i, j);
            let mut not_low_point = false;
            for pp in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let i2 = i as i16 + pp.0;
                let j2 = j as i16 + pp.1;
                if i2 < 0 || i2 >= height as i16 {
                    continue;
                }
                if j2 < 0 || j2 >= width as i16 {
                    continue;
                }
                if map[i2 as usize][j2 as usize] <= map[i][j] {
                    not_low_point = true;
                    break;
                }
            }
            if !not_low_point {
                low_points.push((i as u8, j as u8));
            }
        }
    }

    let mut basin_sizes = Vec::new();
    
    // println!("Low points: {:?}", low_points);
    
    for point in low_points {
        let mut basin_points = HashSet::from([point]);
        basin(point, &mut basin_points, &map, height, width);
        basin_sizes.push(basin_points.len());
    }

    basin_sizes.sort();
    let top_3 = &basin_sizes[basin_sizes.len()-3..basin_sizes.len()];

    let answer = top_3.iter().fold(1, |a, b| a * b);
    // let sum: u128 = low_points.into_iter().map(|p| (map[p.0 as usize][p.1 as usize] + 1) as u128).sum();
    println!("Answer: {}", answer);
}
