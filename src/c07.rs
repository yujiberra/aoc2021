use std::{io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();

    let mut crab_locs: Vec<i32> = stdin.lock().lines().next().unwrap().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
    crab_locs.sort();
    // println!("The list: {:?}", crab_locs);
    // println!("Length = {}; mid = {}", crab_locs.len(), crab_locs.len() /2);

    // println!("Answer: {:?}", &crab_locs[(foo -3)..(foo + 3)]);
    let destination = crab_locs[crab_locs.len() / 2];
    let sum: i32 = crab_locs.iter().map(|l| (destination - l).abs()).sum();
    println!("Answer: {}", sum);
}
