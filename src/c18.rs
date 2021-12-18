use std::{
    io::{self, BufRead}, iter::Peekable,
};

#[derive(Debug)]
enum SNum {
    Entry(u64),
    Pair(Box<SNum>, Box<SNum>)
}

fn parse(input: &str) -> SNum {
    return parse_recurse(&mut input.chars().peekable());
}

fn parse_recurse<I: Iterator<Item = char>>(input: &mut Peekable<I>) -> SNum {
    let char = input.next().unwrap();
    if char.is_numeric() {
        let mut number_chars = Vec::new();
        number_chars.push(char);
        while input.peek().unwrap().is_numeric() {
            number_chars.push(input.next().unwrap());
        }
        return SNum::Entry(number_chars.iter().collect::<String>().parse().unwrap());
    } else if char == '[' {
        let first_number = parse_recurse(input);
        if input.next().unwrap() != ',' {
            panic!();
        }
        let second_number = parse_recurse(input);
        if input.next().unwrap() != ']' {
            panic!();
        }
        return SNum::Pair(Box::new(first_number), Box::new(second_number));

    }
    SNum::Entry(3)
}

fn reduce(number: SNum) -> SNum {
    let mut current_number = number;
    let mut keep_looping = true;
    while keep_looping {
        keep_looping = false;
        let mut keep_exploding = true;
        while keep_exploding {
            let explosion_result = explode(current_number);
            current_number = explosion_result.0;
            keep_exploding = explosion_result.1;
            keep_looping = keep_looping || explosion_result.1;
        }
        let split_result = split(current_number);
        current_number = split_result.0;
        keep_looping = keep_looping || split_result.1;
    }
    return current_number;
}

fn split(number: SNum) -> (SNum, bool) {
    match number {
        SNum::Pair(left_num, right_num) => {
            let (left_num, split_happened) = split(*left_num);
            if split_happened {
                return (SNum::Pair(Box::new(left_num),
                                   Box::new(*right_num)),
                        true);
            } else {
                let (right_num, split_happened) = split(*right_num);
                return (SNum::Pair(Box::new(left_num),
                                   Box::new(right_num)),
                        split_happened);
            }
        }, 
        SNum::Entry(num) => {
            if num >= 10 {
                let half = (num as f64) / 2.0;
                return (SNum::Pair(Box::new(SNum::Entry(half.floor() as u64)),
                                   Box::new(SNum::Entry(half.ceil() as u64))),
                        true);
            } else {
                return (number, false);
            }
        }
    }
}

fn explode(number: SNum) -> (SNum, bool) {
    let result = explode_recurse(number, Some(0), (0, 0));
    // match result.2 {
    //     (0, 0) => (),
    //     _ => println!("Unused explosion output: {:?}", result.2)
    // }
    return (result.0, result.1); 
}

fn explode_recurse(number: SNum, depth: Option<u8>, explosion_input: (u64, u64)) -> (SNum, bool, (u64, u64)) {
    match depth {
        Some(4) => {
            match number {
                SNum::Pair(left_num, right_num) => {
                    if let (SNum::Entry(left_value), SNum::Entry(right_value)) =  (*left_num, *right_num) {
                        return (SNum::Entry(0), true, (left_value, right_value));
                    } else {
                        panic!("Found pair nested more than 4 deep");
                    }
                },
                _ => return (number, false, (0, 0))
            }
        }, 
        Some(n) => {
            match number {
                SNum::Pair(left_num, right_num) => {
                    let (left_result_num, explosion_happened, left_explosion) = explode_recurse(*left_num, Some(n+1), (0, 0));
                    // left side exploded
                    if explosion_happened {
                        let (right_result_num, _, _) = explode_recurse(*right_num, None, (left_explosion.1, 0));
                        return (SNum::Pair(Box::new(left_result_num), Box::new(right_result_num)), true, (left_explosion.0, 0));
                    } else {
                        let (right_result_num, explosion_happened, right_explosion) = explode_recurse(*right_num, Some(n+1), (0, 0));
                        // right side exploded
                        if explosion_happened {
                            let (new_left_result_num, _, _) =  explode_recurse(left_result_num, None, (0, right_explosion.0));
                            return (SNum::Pair(Box::new(new_left_result_num), Box::new(right_result_num)), true, (0, right_explosion.1));
                        // no internal explosions
                        } else {
                            return (SNum::Pair(Box::new(left_result_num), Box::new(right_result_num)), false, (0, 0));
                        }
                    }
                },
                _ => return (number, false, (0, 0))
            }
        },
        None => {
            match number {
                SNum::Pair(left_num, right_num) => {
                    return (SNum::Pair(Box::new(explode_recurse(*left_num, None, (explosion_input.0, 0)).0),
                                       Box::new(explode_recurse(*right_num, None, (0, explosion_input.1)).0)),
                            false,
                            (0, 0));
                }, 
                SNum::Entry(num) => {
                    return (SNum::Entry(num + explosion_input.0 + explosion_input.1), false, (0, 0));
                }
            }
        }
    }
}

fn add(first: SNum, second: SNum) -> SNum {
    return SNum::Pair(Box::new(first), Box::new(second));
}

fn magnitude(number: &SNum) -> u64 {
    match number {
        SNum::Entry(num) => *num,
        SNum::Pair(first, second) => 3 * magnitude(&*first) + 2 * magnitude(&*second),
    }
}
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut sum = parse(&lines.next().unwrap().unwrap());
    for line in lines {
        sum = reduce(add(sum, parse(&line.unwrap())));
    }
    println!("{:?}", sum);
    println!("magnitude = {}", magnitude(&sum));
}
