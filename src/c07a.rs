use std::{io::{self, BufRead}};

fn cost(distance: i32) -> i32 {
    return distance * (distance + 1) / 2;
}

fn main() {
    let stdin = io::stdin();

    let crab_locs: Vec<i32> = stdin.lock().lines().next().unwrap().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
    let sum: i32 = crab_locs.iter().sum();
    let average: f32 = (sum as f32) / (crab_locs.len() as f32);

    let trunc = average.trunc() as i32;
    let candidates = [trunc - 2, trunc - 1, trunc, trunc + 1, trunc + 2];

    let total_cost: Vec<i32> = candidates.iter().map(|c|
        crab_locs.iter()
            .map(|l| cost((l - c).abs()))
            .sum()).collect();

    println!("Locations: {:?}", candidates);
    println!("Answers: {:?}", total_cost);
}
