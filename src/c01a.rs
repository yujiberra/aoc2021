use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut values = [None, None, None, None];
    let mut total = 0;

    for line in stdin.lock().lines() {
        let line_text = line.unwrap();
        let number:i32 = line_text.parse().unwrap();
        for n in 0..3 {
            values[n] = values[n+1];
        }
        values[3] = Some(number);

        if values[0].is_some() && values[3].unwrap() > values[0].unwrap() {
            total += 1; 
        }

    }
    println!("Answer: {}", total);
}
