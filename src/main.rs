use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut previous = i32::MAX;
    let mut total = 0;

    for line in stdin.lock().lines() {
        let line_text = line.unwrap();
        let number :i32 = line_text.parse().unwrap();
        if number > previous {
            total += 1;
        }
        previous = number;
    }
    println!("Answer: {}", total);
}
