use core::panic;
use std::io::{self, BufRead};


fn fun(characters: &mut Vec<char>, total_score: &mut u128, c: char, score: u128) -> bool {
    if characters[characters.len() - 1] == c {
        characters.pop();
        return false;
    } else {
        *total_score = *total_score + score;
        return true;
    }
}

fn main() {
    let stdin = io::stdin();
   
    let mut total_score: u128 = 0;
    for line in stdin.lock().lines() {
        let mut characters: Vec<char> = Vec::new();
        for char in line.unwrap().chars() {
            match char {
                '(' => characters.push(char),
                '[' => characters.push(char),
                '{' => characters.push(char),
                '<' => characters.push(char),
                ')' => {
                    if fun(&mut characters, &mut total_score, '(', 3) {
                        break;
                    }
                },
                ']' => {
                    if fun(&mut characters, &mut total_score, '[', 57) {
                        break;
                    }
                },
                '}' => {
                    if fun(&mut characters, &mut total_score, '{', 1197) {
                        break;
                    }
                },
                '>' => {
                    if fun(&mut characters, &mut total_score, '<', 25137) {
                        break;
                    }
                },
                _ => panic!()
            }
        }
        println!("Current score: {}", total_score);
    }
    println!("Answer: {}", total_score);
}
