use std::fs;

fn read_input() -> Vec<i64> {
    let input = fs::read_to_string("input/aoc07.txt")
        .expect("Unable to read input file");
    input
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn mean(numbers: &Vec<i64>) -> f64 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum as f64 / numbers.len() as f64
}

pub fn aoc7_1() {
    let mut positions = read_input();
    positions.sort();
    let mid = positions.len() / 2;
    let target = positions[mid];
    let fuel = positions
        .iter()
        .fold(0, |acc, x| acc + i64::abs(x - target));
    println!("AOC7.1: {}", fuel);
}

fn fuel_expenditure(crabs: &Vec<i64>, k: i64) -> i64 {
    let mut sum = 0;
    for crab in crabs {
        let sqr = (crab - k) * (crab -k);
        sum += (sqr + i64::abs(crab - k)) / 2;
    }
    sum
}

pub fn aoc7_2() {
    // hot damn, this is cool
    // solution from https://www.reddit.com/r/adventofcode/comments/rawxad/2021_day_7_part_2_i_wrote_a_paper_on_todays/
    // I definitely didn't solve it myself, but followed the same path as others before looking
    // for help. I suppose its still quite good to be able to read and implement a math paper.
    // I feel no remorse.
    let positions = read_input();
    let mean = mean(&positions);
    let pos1 = f64::floor(mean);
    let pos2 = f64::ceil(mean);
    let ans1 = fuel_expenditure(&positions, pos1 as i64);
    let ans2 = fuel_expenditure(&positions, pos2 as i64);
    let answer = if ans1 < ans2 {
        ans1 as i64
    } else {
        ans2 as i64
    };
    println!("AOC7.2: {}", answer);
}