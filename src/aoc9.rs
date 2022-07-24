
fn get_input() -> Vec<Vec<u8>> {
    let raw = include_str!("../input/aoc9.txt");
    let maps = raw
        .trim()
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            acc.push(line
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect()
            );
            acc
        });
    maps
}

fn is_lowpoint(maps: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let mut lowpoint = true;
    // check up down left right
    // up
    if y > 0 {
        if maps[y - 1][x] <= maps[y][x] {
            lowpoint = false;
        }
    }
    // left
    if x > 0 {
        if maps[y][x - 1] <= maps[y][x] {
            lowpoint = false;
        }
    }
    // right
    if x < maps[y].len() - 1 {
        if maps[y][x + 1] <= maps[y][x] {
            lowpoint = false;
        }
    }
    // down
    if y < maps.len() - 1 {
        if maps[y + 1][x] <= maps[y][x] {
            lowpoint = false;
        }
    }
    lowpoint
}

pub fn aoc9_1() {
    let maps = get_input();
    let mut sum = 0;
    for i in 0..maps.len() {
        for j in 0..maps[i].len() {
            if is_lowpoint(&maps, j, i) {
                sum += maps[i][j] as u32 + 1;
            }
        }
    }
    println!("AOC9.1: {}", sum);
}