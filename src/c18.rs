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

fn explode(number: SNum) -> SNum {
    let result = explode_recurse(number, Some(0), (0, 0));
    if result.1.is_some() {
        panic!();
    }
    return result.0; 
}

fn explode_recurse(number: SNum, depth: Option<u8>, explosion_input: (u64, u64)) -> (SNum, Option<(u64, u64)>) {
    match depth {
        Some(4) => {
            println!("Depth 4!");
            match number {
                SNum::Pair(left_num, right_num) => {
                    // left number explodes
                    // let both_sides = (*left_num, *right_num);
                    if let SNum::Pair(ll, lr) = *left_num {
                        // TODO: make right side just any SNum and use recursion
                        if let (SNum::Entry(ll_num), SNum::Entry(lr_num)) =  (*ll, *lr) {
                            let new_left = 0;
                            let new_right = explode_recurse(*right_num, None, (lr_num, 0)).0;
                            return (SNum::Pair(Box::new(SNum::Entry(new_left)), 
                                               Box::new(new_right)), 
                                    Some((ll_num, 0)));
                        } else {
                            panic!("Found pair nested more than 4 deep");
                        }
                    // right number explodes
                    } else if let (SNum::Entry(l), SNum::Pair(rl, rr)) = (*left_num, *right_num) {
                        if let (SNum::Entry(rl_num), SNum::Entry(rr_num)) =  (*rl, *rr) {
                            let new_left = l + rl_num;
                            let new_right = 0;
                            return (SNum::Pair(Box::new(SNum::Entry(new_left)),
                                               Box::new(SNum::Entry(new_right))), 
                                    Some((0, rr_num)) );
                        } else {
                            panic!("Found pair nested more than 4 deep");
                        }
                    } else {
                        return (SNum::Pair(Box::new(*left_num), Box::new(*right_num)), None);
                        // return (SNum::Pair(Box::new(*left_num), Box::new(*right_num)), None);
                    }
                },
                _ => return (number, None)
            }
        }, 
        Some(n) => {
            match number {
                SNum::Pair(left_num, right_num) => {
                    let (left_result_num, left_explosion) = explode_recurse(*left_num, Some(n+1), (0, 0));
                    // left side exploded
                    if let Some((explosion_ll, explosion_lr)) = left_explosion {
                        let (right_result_num, _) = explode_recurse(*right_num, None, (explosion_lr, 0));
                        return (SNum::Pair(Box::new(left_result_num), Box::new(right_result_num)), Some((explosion_ll, 0)));
                    } else {
                        let (right_result_num, right_explosion) = explode_recurse(*right_num, Some(n+1), (0, 0));
                        // right side exploded
                        if let Some((explosion_rl, explosion_rr)) = right_explosion {
                            let (new_left_result_num, _) =  explode_recurse(left_result_num, None, (0, explosion_rl));
                            return (SNum::Pair(Box::new(new_left_result_num), Box::new(right_result_num)), Some((0, explosion_rr)));
                        // no internal explosions
                        } else {
                            return (SNum::Pair(Box::new(left_result_num), Box::new(right_result_num)), None);
                        }
                    }
                },
                _ => return (number, None)
            }
        },
        None => {
            match number {
                SNum::Pair(left_num, right_num) => {
                    return (SNum::Pair(Box::new(explode_recurse(*left_num, None, (explosion_input.0, 0)).0),
                                       Box::new(explode_recurse(*right_num, None, (0, explosion_input.1)).0)),
                            None);
                }, 
                SNum::Entry(num) => {
                    return (SNum::Entry(num + explosion_input.0 + explosion_input.1), None);
                }
            }
        }
    }
    // match number {
    //     //i think this value is supposed to *come* from the left but i'm not sure
    //     SNum::Entry(num) => return (SNum::Entry(*num + left_value), None), 
    //     SNum::Pair(first, second) => {
    //         if depth == 3 {
    //             let left_num = match **first {
    //                 SNum::Pair(_,_) => panic!(),
    //                 SNum::Entry(num) => num,
    //             }; 
    //             let right_num = match **second {
    //                 SNum::Pair(_,_) => panic!(),
    //                 SNum::Entry(num) => num,
    //             }; 
    //             return (SNum::Entry(0), Some((left_num, right_num)));
    //         } else {
    //             let (left_number, left_explosion) = explode_recurse(first, depth+1);
    //             if left_explosion.is_none() {

    //             } else {
    //                 return SNum::Pair()

    //             }

    //             return SNum::Pair(, explode_recurse(second, depth+1))
    //         }
    //     },
    // }

}

fn magnitude(number: &SNum) -> u64 {
    match number {
        SNum::Entry(num) => *num,
        SNum::Pair(first, second) => 3 * magnitude(&*first) + 2 * magnitude(&*second),
    }
}
fn main() {
    let stdin = io::stdin();
    loop {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let number = parse(&line);
        println!("{:?}", number);
        println!("magnitude = {}", magnitude(&number));
        println!("explode once = {:?}", explode(number));
    }
}
