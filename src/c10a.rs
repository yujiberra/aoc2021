use core::panic;
use std::io::{self, BufRead};


fn fun(characters: &mut Vec<char>, c: char) -> bool {
    if characters[characters.len() - 1] == c {
        characters.pop();
        return false;
    } else {
        return true;
    }
}

fn main() {
    let stdin = io::stdin();
   
    let mut scores: Vec<u128> = Vec::new(); 
    for line in stdin.lock().lines() {
        let mut characters: Vec<char> = Vec::new();
        let mut invalid = false;
        for char in line.unwrap().chars() {
            match char {
                '(' => characters.push(char),
                '[' => characters.push(char),
                '{' => characters.push(char),
                '<' => characters.push(char),
                ')' => {
                    if fun(&mut characters, '(') {
                        invalid = true;
                        break;
                    }
                },
                ']' => {
                    if fun(&mut characters, '[') {
                        invalid = true;
                        break;
                    }
                },
                '}' => {
                    if fun(&mut characters, '{') {
                        invalid = true;
                        break;
                    }
                },
                '>' => {
                    if fun(&mut characters, '<') {
                        invalid = true;
                        break;
                    }
                },
                _ => panic!()
            }
        }
        if !invalid {
            let mut score = 0;
            for char in characters.iter().rev() {
                score *= 5;
                match char {
                    '(' => score += 1,
                    '[' => score += 2,
                    '{' => score += 3,
                    '<' => score += 4,
                    _ => panic!()
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("Scores: {:?}", scores);
    println!("Answer: {}", scores[(scores.len() / 2)]);
}
