use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone)]
struct OctopusState {
    energy_level: u8,
    flashed: bool,
}

fn main() {
    let stdin = io::stdin();

    let mut grid: [[OctopusState; 10]; 10] = [[OctopusState { energy_level: 0, flashed: false}; 10]; 10];

    let mut i = 0;
    for line in stdin.lock().lines() {
        let mut j = 0;
        for char in line.unwrap().chars() {
            grid[i][j].energy_level = char.to_digit(10).unwrap() as u8;
            j += 1;
        }
        i += 1;
    }

    let mut total_flashes = 0;

    for counter in 0..100000 {
        for i in 0..10 {
            for j in 0..10 {
                grid[i][j].energy_level += 1;
            }
        }
        let mut flash_count = 0;
        let mut should_check_flashes = true;
        while should_check_flashes {
            should_check_flashes = false;
            for i in 0..10 {
                for j in 0..10 {
                    if grid[i][j].energy_level > 9 && !grid[i][j].flashed {
                        println!("flashing at {}, {}", i, j);
                        should_check_flashes = true;
                        flash_count += 1;
                        grid[i][j].flashed = true;
                        total_flashes += 1;
                        for i_inc in 0..3 {
                            for j_inc in 0..3 {
                                let ii  = ((i + i_inc) as i8) - 1;
                                let jj = ((j + j_inc) as i8) - 1;
                                if ii >= 0 && ii < 10 && jj >= 0 && jj < 10 {
                                    grid[ii as usize][jj as usize].energy_level += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        if flash_count == 100 {
            println!("Done on step {}", counter + 1);
            return;
        }
        for i in 0..10 {
            for j in 0..10 {
                if grid[i][j].flashed {
                    grid[i][j].energy_level = 0;
                    grid[i][j].flashed = false;
                }
                print!("{}", grid[i][j].energy_level);
            }
            println!();
        }
    }

    println!("Answer: {}", total_flashes);
}
