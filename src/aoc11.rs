struct Octopus {
    energy: u8,
    flashed: bool,
}

impl Octopus {
    pub fn new(energy: u8) -> Octopus {
        Octopus {
            energy,
            flashed: false,
        }
    }
}

fn read_input() -> Vec<Vec<Octopus>> {
    include_str!("../input/aoc11.txt")
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| {
                    Octopus::new(c
                        .to_digit(10)
                        .unwrap() as u8
                    )
                })
                .collect()
        })
        .collect()
}

/*
fn print_grid(grid: &Vec<Vec<Octopus>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell.energy);
        }
        println!();
    }
    println!();
}
*/

fn flash(grid: &mut Vec<Vec<Octopus>>) -> i32 {
    // shamelessly ripped off from https://github.com/Jomy10/Advent-Of-Code-2021/blob/master/day11/src/main.rs
    // I could not figure puzzle out for the life of me, plus my flash function used a stack and
    // flashed everything at once.
    let mut flashes = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col].energy > 9 && !grid[row][col].flashed {
                flashes += 1;
                grid[row][col].flashed = true;
                (0..=2 as i32).map(|val| val - 1).for_each(|radd| {
                    (0..=2 as i32).map(|val| val - 1).for_each(|cadd| {
                        let r = row as i32 + radd;
                        let c = col as i32 + cadd;
                        if !(r < 0 || c < 0 || r >= grid.len() as i32 || c >= grid[0].len() as i32) {
                            grid[r as usize][c as usize].energy += 1;
                        }
                    });
                });
            }
        }
    }
    flashes
}

pub fn aoc11_1() {
    let mut input = read_input();
    let mut flashes: i64 = 0;
    for _ in 0..100 {
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                input[j][i].energy += 1;
            }
        }
        loop {
            let flashes_this_step = flash(&mut input);
            flashes += flashes_this_step as i64;
            if flashes_this_step == 0 {
                break;
            }
        }
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[j][i].flashed {
                    input[j][i].flashed = false;
                    input[j][i].energy = 0;
                }
            }
        }
    }
    println!("AOC11_1: {}", flashes);
}

pub fn aoc11_2() {
    let mut input = read_input();
    let total_ocupi: u64 = (input.len() * input[0].len()) as u64;
    let mut sync_step: u64 = 0;
    for k in 1..usize::MAX {
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                input[j][i].energy += 1;
            }
        }
        let mut flashes: u64 = 0;
        loop {
            let flashes_this_step = flash(&mut input);
            flashes += flashes_this_step as u64;
            if flashes_this_step == 0 {
                sync_step = k as u64;
                break;
            }
        }
        if flashes == total_ocupi {
            break;
        }
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[j][i].flashed {
                    input[j][i].flashed = false;
                    input[j][i].energy = 0;
                }
            }
        }
    }
    println!("AOC11_2: {}", sync_step);
}
