use std::collections::{HashSet};
use std::io::{self, BufRead};
use std::cmp;

type Point = (i32, i32);

#[derive(Clone, Debug)]
struct Line (Point, Point);

impl Line {
    fn is_horizontal(&self) -> bool {
        return self.0.1 == self.1.1;
    }

    fn is_vertical(&self) -> bool {
        return self.0.0 == self.1.0;
    }

    fn is_orthogonal(&self) -> bool {
        return self.is_horizontal() || self.is_vertical();
    }

    fn vertical_overlap(&self, other: &Line) -> (i32, i32) {
        let lower = cmp::max(cmp::min(self.0.1, self.1.1), cmp::min(other.0.1, other.1.1));
        let higher = cmp::min(cmp::max(self.0.1, self.1.1), cmp::max(other.0.1, other.1.1));
        return (lower, higher);
    }

    fn horizontal_overlap(&self, other: &Line) -> (i32, i32) {
        let lower = cmp::max(cmp::min(self.0.0, self.1.0), cmp::min(other.0.0, other.1.0));
        let higher = cmp::min(cmp::max(self.0.0, self.1.0), cmp::max(other.0.0, other.1.0));
        return (lower, higher);
    }

    fn intersection(&self, other: Line) -> Vec<Point> {
        // println!("checking intersection of {:?} and {:?}", self, &other);
        if self.is_horizontal() && other.is_vertical() {
            return orthogonal_intersection(self, &other);
        } else if self.is_vertical() && other.is_horizontal() {
            return orthogonal_intersection(&other, self);
        } else if self.is_vertical() && other.is_vertical() && self.0.0 == other.0.0 {
            // println!("Double vertical");
            let mut intersection = Vec::new();
            let (lower, higher) = self.vertical_overlap(&other);
            if lower <= higher {
                for i in lower..higher+1 {
                    intersection.push((self.0.0, i));
                }
            }
            return intersection;
        } else if self.is_horizontal() && other.is_horizontal() && self.0.1 == other.0.1 {
            // println!("Double horizontal");
            let mut intersection = Vec::new();
            let (lower, higher) = self.horizontal_overlap(&other);
            if lower <= higher {
                for i in lower..higher+1 {
                    intersection.push((i, self.0.1));
                }
            }
            return intersection;
        } else if !self.is_orthogonal() && ! other.is_orthogonal() {
            println!("checking party on {:?} and {:?}", self, &other);
            println!("parity = {}, {}", (self.0.0 + self.0.1) % 2, (other.0.0 + other.0.1) % 2);
            if (self.0.0 + self.0.1) % 2 == (other.0.0 + other.0.1) % 2 {
                let vert = self.vertical_overlap(&other);
                let horz = self.horizontal_overlap(&other);
                println!("vert = {:?}, horz = {:?}", vert, horz);
                if vert.0 <= vert.1 && horz.0 <= horz.1 {
                    println!("zones match");
                    let slope_1: i32 = (self.0.1 - self.1.1) / (self.0.0 - self.1.0);
                    let slope_2: i32 = (other.0.1 - other.1.1) / (other.0.0 - other.1.0);
                    let x: i32 = (other.0.1 - self.0.1 + slope_1 * self.0.0 - slope_2 * other.0.0) / (slope_1 - slope_2);
                    let y: i32 = slope_1 * (x - self.0.0) + self.0.1;
                    return vec![(x, y)];
                }
            }
        }
        return Vec::new();
    }
}

fn orthogonal_intersection(horizontal: &Line, vertical: &Line) -> Vec<Point> {
    if horizontal.0.1 >= cmp::min(vertical.0.1, vertical.1.1) &&
        horizontal.0.1 <= cmp::max(vertical.0.1, vertical.1.1) &&
        vertical.0.0 >= cmp::min(horizontal.0.0, horizontal.1.0) &&
        vertical.0.0 <= cmp::max(horizontal.0.0, horizontal.1.0) {
        return vec![(vertical.0.0, horizontal.0.1)];
    }
    return Vec::new();
}

fn point_from_string(input: &str) -> Point {
    let mut splits = input.split(",");
    return (splits.next().unwrap().parse().unwrap(), splits.next().unwrap().parse().unwrap());
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

    let mut intersections = HashSet::new();
    for i in 0..lines.len() {
        for j in i+1..lines.len() {
            for (x,y) in lines[i].intersection(lines[j].clone()) {
                intersections.insert((x, y));
            }
        }
    }

    // println!("Intersections: {:?}", intersections);
    println!("Count: {:?}", intersections.len());

}
