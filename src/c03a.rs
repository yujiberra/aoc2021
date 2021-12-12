use std::{io::{self, BufRead}};

fn fun(values: &Vec<i32>, length: usize, bit_check: i32) -> i32 {
    let mut current_values: Vec<i32> = values.clone();
    for j in 0..length {
        println!("{:?}", current_values);
        let half = (current_values.len() as i32) / 2;
        let even = current_values.len() % 2 == 0;

        let k = length - 1 - j;
        let sum:i32 = current_values.iter().map(|value| ((1 << k) & value) >> k).sum();

        let bit;

        if sum > half || (even && sum == half) {
            bit = bit_check;
        } else {
            bit = 1 - bit_check;
        }
        println!("k = {}, sum = {}, bit = {}", k, sum, bit);
        current_values = current_values.iter().filter(|&value| (((1 << k) & value) >> k) == bit).map(|&x| x).collect();

        if current_values.len() == 1 {
            return current_values[0];
        }
    }
    return -1;
}
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

    let oxygen = fun(&values, length, 1);
    let co2 = fun(&values, length, 0);
    
    println!("oxygen: {}, co2: {}", oxygen, co2);
    println!("Answer: {}", oxygen * co2);
}
