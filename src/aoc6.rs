use std::fs;

struct LanternFish {
    state: i32,
}

impl LanternFish {
    fn advance(&mut self) -> Option<LanternFish> {
        return if self.state == 0 {
            self.state = 6;
            Some(LanternFish { state: 8 })
        } else {
            self.state -= 1;
            None
        }
    }
}

impl Clone for LanternFish {
    fn clone(&self) -> Self {
        LanternFish { state: self.state }
    }
}

fn read_input() -> Vec<LanternFish> {
    let input = fs::read_to_string("input/aoc06.txt").expect("Error reading input file");
    let initial_state = input
        .split(',')
        .map(|x| x.parse::<i32>().expect(x))
        .collect::<Vec<i32>>();
    initial_state.iter().map(|x| LanternFish { state: *x }).collect::<Vec<LanternFish>>()
}

fn read_input_u() -> Vec<u8> {
    let input = fs::read_to_string("input/aoc06.txt").expect("Error reading input file");
    input
        .split(',')
        .map(|x| x.parse::<u8>().expect(x))
        .collect::<Vec<u8>>()
}

fn how_many_fish(days: u32, fish: &Vec<LanternFish>) -> u32 {
    let mut fishes = fish.clone();
    for _ in 0..days {
        let mut new_fish = Vec::new();
        for f in &mut fishes {
            if let Some(new_f) = f.advance() {
                new_fish.push(new_f);
            }
        }
        fishes.extend(new_fish);
    }
    fishes.len() as u32
}

pub fn aoc6_1() {
    let fishes = read_input();
    println!("AOC6.1: {}", how_many_fish(80, &fishes));
}

pub fn aoc6_2() {
    let raw_input = read_input_u();
    let mut fishes: Vec<u64>= vec![0; 9];
    for fish in raw_input {
        fishes[fish as usize] += 1;
    }
    for _ in 0..256 {
        let mut repro = 0;
        for j in 0..fishes.len() {
            if j == 0 {
                repro = fishes[j];
            } else {
                fishes[j-1] = fishes[j];
            }
        }
        fishes[6] += repro;
        fishes[8] = repro;

    }
    let count: u64 = fishes.iter().sum();
    println!("AOC6.2: {}", count);
}