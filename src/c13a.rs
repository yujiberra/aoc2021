use std::{io::{self, BufRead}};

// fn fold(points: &mut Vec<(u16, u16)>, axis: &str, fold_location: u16) {
fn fold(points: &mut Vec<(u16, u16)>, fold: (String, u16)) {
    for i in 0..points.len() {
        let p = points[i];
        match fold.0.as_str() {
            "x" => points[i] = (flip(p.0, fold.1), p.1),
            "y" => points[i] = (p.0, flip(p.1, fold.1)),
            _ => panic!()
        }
    }
    points.sort();
    points.dedup();
}

fn flip(coordinate: u16, fold_location: u16) -> u16 {
    if fold_location < coordinate {
        return fold_location - (coordinate - fold_location);
    } else {
        return coordinate;
    }
}

fn main() {
    let stdin = io::stdin();

    let mut points: Vec<(u16, u16)> = Vec::new();

    for line in stdin.lock().lines() {
        let unwrapped = line.unwrap();
        if unwrapped.len() == 0 {
            break;
        }
        let mut splits = unwrapped.split(",");
        let first: u16 = splits.next().unwrap().parse().unwrap();
        let second: u16 = splits.next().unwrap().parse().unwrap();
        points.push((first, second));
    }

    // println!("points {:?}", points);

    let mut folds: Vec<(String, u16)> = Vec:: new();

    for line in stdin.lock().lines() {
        let mut unwrapped = line.unwrap();
        if unwrapped.len() == 0 {
            break;
        }
        unwrapped.drain(..11);
        let mut splits = unwrapped.split("=");
        let axis = String::from(splits.next().unwrap());
        let coordinate: u16 = splits.next().unwrap().parse().unwrap();
        folds.push((axis, coordinate));
    }

    // println!("folds {:?}", folds);

    folds.iter().for_each(|f| fold(&mut points, f.clone()));
    println!("points {:?}", points);
    println!("points count {:?}", points.len());

    let mut grid: [[char; 39]; 6] = [[' '; 39]; 6];
    points.iter().for_each(|p| grid[p.1 as usize][p.0 as usize] = '*');
    for row in grid {
        let s: String = row.into_iter().collect();
        println!("{}", s);
    }
}
