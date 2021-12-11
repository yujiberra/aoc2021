use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut horizontal = 0;
    let mut depth = 0;

    for line in stdin.lock().lines() {
        let line_text = line.unwrap();
        let mut split = line_text.as_str().split_whitespace();
        let first = split.next();
        let second = split.next().unwrap();
        let number :i32 = second.parse().unwrap();

        match first {
            Some("forward") => horizontal += number,
            Some("down") => depth += number,
            Some("up") => depth -= number,
            _ => panic!()
        }
        
    }
    println!("Answer: {}", horizontal * depth);
}
