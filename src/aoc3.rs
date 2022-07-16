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

pub fn aoc3_1() {
    let input = read_input();
    let mut ones = vec![0; input[0].len()];
    for line in input {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            } else {
                ones[i] -= 1;
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for c in ones {
        if c > 0 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();
    println!("AOC3.1: {}", gamma * epsilon);
}