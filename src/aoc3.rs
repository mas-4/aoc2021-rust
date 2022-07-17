use std::fs;
use std::usize;

fn read_input() -> Vec<String> {
    fs::read_to_string("input/aoc3.txt")
        .expect("Failed to read input file")
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect()
}

fn get_frequency(input: &Vec<String>) -> Vec<i64> {
    let mut ones = vec![0; input[0].len()];
    for line in input {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            } else if c == '0' {
                ones[i] -= 1;
            } else {
                panic!("Invalid character");
            }
        }
    }
    ones
}

fn get_gamma_binary(input: &Vec<String>) -> String {
    // gamma is most common for each index, defaulting to 1
    let ones = get_frequency(input);
    let mut binary = String::new();
    for i in 0..ones.len() {
        if ones[i] >= 0 {
            binary.push('1');
        } else {
            binary.push('0');
        }
    }
    binary
}

pub fn aoc3_1() {
    let input = read_input();
    let gamma = get_gamma_binary(&input);
    let mut epsilon = String::new();
    for c in gamma.chars() {
        if c == '1' {
            epsilon.push('0');
        } else {
            epsilon.push('1');
        }
    }
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();
    println!("AOC3.1: {}", gamma * epsilon);
}

pub fn aoc3_2() {
    let input = read_input();
    let mut oxygen = input.clone();
    let mut co2 = input.clone();
    // Holy cow this took forever, I didn't realize it was a moving frequency, I assumed the
    // initial frequency was static.
    for i in 0..input[0].len() {
        if oxygen.len() > 1 {
            let frequencies = get_frequency(&oxygen);
            let val = if frequencies[i] >= 0 { '1' } else { '0' };
            oxygen = oxygen
                .iter()
                .filter(|x| x.chars().nth(i).unwrap() == val)
                .cloned()
                .collect()
        }
        if co2.len() > 1 {
            let frequencies = get_frequency(&co2);
            let val = if frequencies[i] >= 0 { '0' } else { '1' };
            co2 = co2
                .iter()
                .filter(|x| x.chars().nth(i).unwrap() == val)
                .cloned()
                .collect()
        }

        if oxygen.len() == 1 && co2.len() == 1 {
            break;
        }
    }
    let oxygen = usize::from_str_radix(&oxygen[0], 2).unwrap();
    let co2 = usize::from_str_radix(&co2[0], 2).unwrap();
    println!("AOC3.2: {}", oxygen * co2);
}