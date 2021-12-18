use std::{
    io::{self, BufRead}, iter::Peekable,
};

#[derive(Debug)]
enum SnailfishNumber {
    Entry(u64),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>)
}

fn parse(input: &str) -> SnailfishNumber {
    return parse_recurse(&mut input.chars().peekable());
}

fn parse_recurse<I: Iterator<Item = char>>(input: &mut Peekable<I>) -> SnailfishNumber {
    let char = input.next().unwrap();
    if char.is_numeric() {
        let mut number_chars = Vec::new();
        number_chars.push(char);
        while input.peek().unwrap().is_numeric() {
            number_chars.push(input.next().unwrap());
        }
        return SnailfishNumber::Entry(number_chars.iter().collect::<String>().parse().unwrap());
    } else if char == '[' {
        let first_number = parse_recurse(input);
        if input.next().unwrap() != ',' {
            panic!();
        }
        let second_number = parse_recurse(input);
        if input.next().unwrap() != ']' {
            panic!();
        }
        return SnailfishNumber::Pair(Box::new(first_number), Box::new(second_number));

    }
    SnailfishNumber::Entry(3)
}

fn magnitude(number: SnailfishNumber) -> u64 {
    match number {
        SnailfishNumber::Entry(num) => num,
        SnailfishNumber::Pair(first, second) => 3 * magnitude(*first) + 2 * magnitude(*second),
    }
}
fn main() {
    let stdin = io::stdin();
    loop {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let number = parse(&line);
        println!("{:?}", number);
        println!("magnitude = {}", magnitude(number));
    }
}
