use std::{io::{self, BufRead}, collections::HashMap};

#[derive(Debug)]
struct InsertionRule {
   pair: String,
   element: char, 
}

#[derive(Debug)]
struct Insertion {
    index: usize,
    element: char,
}

fn count_elements(polymer: &String) -> Vec<(char, u32)> {
    let mut counts = HashMap::new();
    polymer.chars().for_each(|c| {
        let existing_value = *counts.get(&c).unwrap_or(&0);
        counts.insert(c, existing_value + 1);
    });
    let mut count_list: Vec<(char, u32)> = counts.iter().map(|c| (*c.0, *c.1)).collect();
    count_list.sort_by_key(|c| c.1);
    return count_list;
}

fn add_step(polymer: &mut String, rules: &Vec<InsertionRule>) {
    let mut insertions: Vec<Insertion> = rules.iter()//.enumerate()
        .flat_map(|rule| {
            (0..polymer.len()-1).map(|i| {
                if &polymer[i..i+2] == &rule.pair {
                    return Some(Insertion {
                        index: i,
                        element: rule.element, 
                    });
                } else {
                    return None;
                }
            })
        })
        .filter_map(|x| x)
        .collect();
    insertions.sort_by_key(|i| i.index);
    let mut offset: usize = 1;
    // println!("additions: {:?}", insertions);
    for insertion in insertions {
        polymer.insert(insertion.index + offset, insertion.element);
        offset += 1;
    } 
    // println!("after insertion: {:?}", polymer);
}

fn main() {
    let stdin = io::stdin();
    
    let mut lines = stdin.lock().lines();

    let mut polymer = lines.next().unwrap().unwrap();

    let rules: Vec<InsertionRule> = lines.skip(1)
        .filter_map(Result::ok)
        .map(|s| {
            let mut splits = s.split(" -> ");
            return InsertionRule {
                pair: String::from(splits.next().unwrap()),
                element: splits.next().unwrap().chars().next().unwrap(),
            }
        })
        .collect();

    // println!("{:?}", rules);

    for i in 0..10 {
        add_step(&mut polymer, &rules);
        // println!("{}", polymer);
        // println!("{}; len = {}", i, polymer.len());
    }

    let counts = count_elements(&polymer);
    // println!("{:?}", counts);
    println!("Answer: {}", counts[counts.len()-1].1 as i64 - counts[0].1 as i64);
}
