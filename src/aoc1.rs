use std::fs;

fn read_input() -> Vec<i32> {
    let contents = fs::read_to_string("input/aoc1.txt").expect( "Couldn't open." );
    let numbers = contents
        .trim()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    numbers
}

pub fn aoc1_1() {
    let numbers = read_input();
    let mut increased = 0;
    let mut last = numbers[0];
    for i in numbers[1..].iter() {
        if *i > last {
            increased += 1;
        }
        last = *i;
    }
    println!("AOC1.1: {}", increased);
}

pub fn aoc1_2() {
    let numbers = read_input();
    let mut first = numbers[0];
    let mut second = numbers[1];
    let mut third = numbers[2];
    let mut sum = first + second + third;
    let mut increased = 0;
    for i in numbers[3..].iter() {
        if second + third + i > sum {
            increased += 1;
        }
        first = second;
        second = third;
        third = *i;
        sum = first + second + third;
    }
    println!("AOC1.2: {}", increased);
}