use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let length;
    let mut values = Vec::new();

    let first_line = stdin.lock().lines().next();
    match first_line {
        Some(Ok(val)) => {
            length = val.len();
            values.push(i32::from_str_radix(&val, 2).unwrap());
        },
        _ => panic!(),
    }

    for line in stdin.lock().lines() {
        values.push(i32::from_str_radix(&line.unwrap(),  2).unwrap());
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    let half = (values.len() as i32) / 2;

    for k in 0..length {
        // println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
        let sum:i32 = values.iter().map(|value| ((1 << k) & value) >> k).sum();
        // println!("Sum: {}", sum);
        if sum > half {
            gamma = gamma | (1 << k);
        } else {
            epsilon = epsilon | (1 << k);
        }
    }

    // println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
    println!("Answer: {}", gamma * epsilon);
}
