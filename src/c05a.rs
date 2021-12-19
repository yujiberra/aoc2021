use std::collections::{HashMap};
use std::hash::Hash;
use std::io::{self, BufRead};
use std::cmp;

use num_traits::Num;

type Point = (i32, i32);

#[derive(Clone, Debug)]
struct Line (Point, Point);

impl Line {
    fn is_vertical(&self) -> bool {
        return self.0.0 == self.1.0;
    }

    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        if self.is_vertical() {
            let y_min = cmp::min(self.0.1, self.1.1);
            let y_max = cmp::max(self.0.1, self.1.1);
            for y in y_min..(y_max + 1) {
                points.push((self.0.0, y));
            }
        } else {
            let slope = (self.1.1 - self.0.1) / (self.1.0 - self.0.0);
            let x_min = cmp::min(self.0.0, self.1.0);
            let x_max = cmp::max(self.0.0, self.1.0);
            let intercept;
            if self.0.0 < self.1.0 {
                intercept = self.0.1;
            } else {
                intercept = self.1.1;
            }
            for x in x_min..x_max+1 {
                points.push((x, (x-x_min) * slope + intercept))
            }
        }
        return points;
    }

    
}

fn point_from_string(input: &str) -> Point {
    let mut splits = input.split(",");
    return (splits.next().unwrap().parse().unwrap(), splits.next().unwrap().parse().unwrap());
}

fn add_or_insert<K: Eq+Hash, V: Num+Copy>(hash_map: &mut HashMap<K, V>, key: K, value: V) {
    let existing_value = *hash_map.get(&key).unwrap_or(&V::zero());
    hash_map.insert(key, existing_value + value);
}
fn main() {

    let lines: Vec<Line> = io::stdin().lock().lines()
        .filter_map(Result::ok)
        .map(|s| {
            let mut splits = s.split(" -> ");
            return Line(point_from_string(splits.next().unwrap()), 
                        point_from_string(splits.next().unwrap())); 
        })
        .collect();

    // println!("Orthogonal lines: {:?}", orthogonal);

    let mut points: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        for point in line.points() {
            add_or_insert(&mut points, point, 1);
        }
    }

    let count = points.values().filter(|v| v > &&1).count();

    // println!("Intersections: {:?}", intersections);
    println!("Count: {:?}", count);

    // for i in 0..9 {
    //     for j in 0..9 {
    //         print!("{}", points.get(&(j, i)).map(|c| c.to_string()).unwrap_or(String::from(".")));

    //     }
    //     println!();
    // }
}
