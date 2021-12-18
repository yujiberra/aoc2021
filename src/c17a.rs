use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut splits = line.split(" ");
    splits.next();
    splits.next();
    let mut x_part = splits.next().unwrap();
    x_part = &x_part[2..x_part.len() - 1];
    let y_part = &splits.next().unwrap()[2..];

    let x_nums: Vec<i32> = x_part.split("..").map(|s| s.parse().unwrap()).collect();
    let y_nums: Vec<i32> = y_part.split("..").map(|s| s.parse().unwrap()).collect();
    let x_range: (i32, i32) = (x_nums[0], x_nums[1]);
    let y_range: (i32, i32) = (y_nums[0], y_nums[1]);

    println!("{:?}", x_range);
    println!("{:?}", y_range);

    let min_x_vel: i32 = 1; //(x_range.0 as f32 * 2.0).sqrt().trunc() as i32;
    let max_x_vel: i32 = x_range.1; //(x_range.0 as f32 / 2.0).trunc() as i32 + 3;

    let mut possible_times: HashMap<i32, Vec<i32>> = HashMap::new();

    for initial_v in min_x_vel..(max_x_vel + 1) {
        let mut t = 0;
        let mut x = 0;
        let mut v = initial_v;
        while v >= 0 && x <= x_range.1 {
            if x >= x_range.0 && x <= x_range.1 {
                // println!("t={}, x={}, v={}, i_v={}", t, x, v, initial_v);
                if !possible_times.contains_key(&t) {
                    possible_times.insert(t, Vec::new());
                }
                let mut velocities_for_t = possible_times.get(&t).unwrap().clone();
                velocities_for_t.push(initial_v);
                possible_times.insert(t, velocities_for_t);
            }
            if v == 0 && x >= x_range.0 && x <= x_range.1 {
                println!("Doing the thing at t={}, v={}, x={}", t, v, x);
                for new_t in (t+1)..(t+300) {
                    if !possible_times.contains_key(&new_t) {
                        possible_times.insert(new_t, Vec::new());
                    }
                    let mut velocities_for_t = possible_times.get(&new_t).unwrap().clone();
                    velocities_for_t.push(initial_v);
                    possible_times.insert(new_t, velocities_for_t);
                }
            }            
            t += 1;
            x += v;
            v -= 1;

        }
    }

    println!("{:?}", possible_times);

    // panic!();

    let mut times: Vec<i32> = possible_times.keys().cloned().collect();
    times.sort();
    let time_range = (times[0], times[times.len() - 1]);

    println!("time range = {:?}", time_range);

    let mut velocity_pairs: Vec<(i32, i32)> = Vec::new();

    for initial_v in y_range.0..(-y_range.0) {
        let mut y = 0;
        let mut t = 0;
        let mut v = initial_v;
        println!("initial v={}", v);
        // let mut hits_zone = false;
        // let mut max_height = 0;
        // let mut hit_zone_t = 0;
        while y >= y_range.0 {
            if y >= y_range.0 && y <= y_range.1 {
                println!("Found!");
                // for hit_time in 1..(&t + 1) {
                //     let x_velocities = possible_times.get(&hit_time);
                //     match x_velocities {
                //         Some(velocities) => {
                //             for x_vel in velocities {
                //                 println!("Adding x_vel={}", *x_vel);
                //                 velocity_pairs.push((*x_vel, initial_v));
                //             }
                //         },
                //         _ => {}
                //     }
                // }
                if possible_times.contains_key(&t) {
                    let x_velocities = possible_times.get(&t).unwrap();
                    for x_vel in x_velocities {
                        println!("Adding x_vel={}", *x_vel);
                        velocity_pairs.push((*x_vel, initial_v));
                    }
                } else {
                    if t > time_range.1 {
                        println!("AOEUSNTHAOEUNTHAOESNUHAOESNUHAOESUNHAOSENUASOEUNAOESNTHAOSENUH");
                        let x_velocities = possible_times.get(&time_range.1).unwrap();
                        for x_vel in x_velocities {
                            println!("Adding x_vel={}", *x_vel);
                            velocity_pairs.push((*x_vel, initial_v));
                        }
                    }
                }
            }
            y += v;
            t += 1;
            v -= 1;
            println!("t={}, v={}, y={}", t, v, y);
        }
        // if hits_zone && max_height > best_height {
        //     best_height = max_height;
        //     best_y_vel = initial_v;
        //     best_t = hit_zone_t;
        //     best_x_vel = *possible_times.get(&best_t).unwrap();
        //     println!("Found: height={} at t={} with xvel={}, yvel={}", best_height, best_t, best_x_vel, best_y_vel);
        // }
        // initial_v = initial_v + 1;
        // if y >= y_range.1 {
        //     break;
        // }
    }

    let unique_pairs: HashSet<(i32, i32)> = HashSet::from_iter(velocity_pairs);

    println!("Pairs: {:?}", unique_pairs);
    println!("Answer: {:?}", unique_pairs.len());
    // println!("Answer: height={} at t={} with xvel={}, yvel={}", best_height, best_t, best_x_vel, best_y_vel);
}
