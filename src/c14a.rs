use std::{io::{self, BufRead}, collections::HashMap, hash::Hash};
use num_traits::Num;

#[derive(Debug)]
struct InsertionRule {
   pair: String,
   element: char, 
}

type Polymer = HashMap<String, u64>;
fn add_or_insert<K: Eq+Hash, V: Num+Copy>(hash_map: &mut HashMap<K, V>, key: K, value: V) {
    let existing_value = *hash_map.get(&key).unwrap_or(&V::zero());
    hash_map.insert(key, existing_value + value);
}

fn count_elements(polymer: Polymer) -> Vec<(char, u64)> {
    let mut counts: HashMap<char, f64>  = HashMap::new();
    for (pair, count) in polymer.iter() {
        for char in pair.chars() {
            add_or_insert(&mut counts, char, *count as f64);
        }
    }
    let mut sorted_counts = Vec::new();
    for (key , value) in counts.iter() {
        sorted_counts.push((*key, (*value / 2.0).ceil() as u64));
    }
    sorted_counts.sort_by_key(|e| e.1);
    return sorted_counts;
}

fn add_step(polymer: Polymer, rules: &Vec<InsertionRule>) -> Polymer {
    let mut new_polymer = Polymer::new();
    for rule in rules {
        let count = *polymer.get(&rule.pair).unwrap_or(&0);

        let mut first_result = rule.pair[0..1].to_owned();
        first_result.push(rule.element);
        add_or_insert(&mut new_polymer, first_result, count);

        let mut second_result = rule.element.to_string();
        second_result.push_str(&rule.pair[1..2]);
        add_or_insert(&mut new_polymer, second_result, count);
    }
    return new_polymer;
}

fn main() {
    let stdin = io::stdin();
    
    let mut lines = stdin.lock().lines();

    let polymer_string = lines.next().unwrap().unwrap();

    let mut polymer = Polymer::new();
    for i in 0..polymer_string.len()-1 {
        let pair = &polymer_string[i..i+2];
        add_or_insert(&mut polymer, String::from(pair), 1);
    }

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

    for i in 0..40 {
        polymer = add_step(polymer, &rules);
        // println!("{}", polymer);
        // println!("{}; len = {}", i, polymer.len());
    }

    let counts = count_elements(polymer);
    println!("{:?}", counts);
    println!("Answer: {}", counts[counts.len()-1].1 as i64 - counts[0].1 as i64);
}
