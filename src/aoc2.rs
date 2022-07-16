use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Instruction {
    direction: Direction,
    distance: i32,
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let idx = input.find(' ').unwrap();
        let (direction, distance) = input.split_at(idx);
        let direction = match direction {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => panic!("Unknown direction: {}", direction),
        };
        let distance = distance.trim().parse::<i32>().unwrap();
        Ok(Instruction { direction, distance, })
    }
}

fn read_input() -> Vec<Instruction> {
    let contents = fs::read_to_string("input/aoc2.txt").expect("Couldn't open.");
    let instructions = contents
        .trim()
        .split("\n")
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();
    instructions
}

pub fn aoc2_1() {
    let instructions = read_input();
    let mut x = 0;
    let mut y = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Up => y -= instruction.distance,
            Direction::Down => y += instruction.distance,
            Direction::Forward => x += instruction.distance,
        }
    }
    println!("AOC2.1: {}", x * y);
}