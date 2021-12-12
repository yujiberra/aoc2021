use std::io::{self, BufRead};
use std::iter;

fn main() {
    let stdin = io::stdin();

    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut fish: Vec<i32> = line.split(",").into_iter().map(|s| s.parse().unwrap()).collect();

    println!("Initial state: {:?}", fish);
    for counter in 0..80 {
        let mut new_fish_count = 0;
        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish[i] = 6;
                new_fish_count += 1;
            } else {
               fish[i] -= 1;
            }
        }
        fish.append(&mut iter::repeat(8).take(new_fish_count).collect());
        println!("After {} days: {:?}", counter + 1, fish.len());
    }
    println!("Answer: {}", fish.len());
}
