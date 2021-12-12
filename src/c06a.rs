use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut fish: [i128; 10] = [0; 10]; 
    let line = stdin.lock().lines().next().unwrap().unwrap();
    for f in line.split(",").into_iter() {
        let index: usize = f.parse().unwrap();
        fish[index] += 1;
    }
    println!("Initial state: {:?}", fish);

    for counter in 0..256 {
        fish[9] = fish[0];
        for i in 0..9 {
            fish[i] = fish[i+1];
        }
        fish[9] = 0;
        fish[6] += fish[8];
        println!("After {} days: {:?}", counter + 1, fish);
    }

    println!("Answer: {}", fish.iter().sum::<i128>());
}
