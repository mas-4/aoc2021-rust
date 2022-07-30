use std::collections::HashSet;

struct Basin {
    xs: Vec<usize>,
    ys: Vec<usize>,
    depths: Vec<u8>,
}

impl Basin {
    fn add_point(&mut self, x: usize, y: usize, depth: u8) {
        self.xs.push(x);
        self.ys.push(y);
        self.depths.push(depth);
    }
}


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

fn traverse(
    maps: &Vec<Vec<u8>>,
    basin: &mut Basin,
    visited: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) {
    // check up down left right
    // if up down left or right is greater than but less than 9, add to basin
    // else if up down left or right is 9 or less than current, skip
    let mut stack = vec![(x, y, maps[y][x])];
    while !stack.is_empty() {
        let (x, y, depth) = stack.pop().unwrap();
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        basin.add_point(x, y, depth);
        // up
        if y > 0 {
            if maps[y - 1][x] > depth && maps[y - 1][x] < 9 {
                stack.push((x, y - 1, maps[y - 1][x]));
            }
        }
        // left
        if x > 0 {
            if maps[y][x - 1] > depth && maps[y][x - 1] < 9 {
                stack.push((x - 1, y, maps[y][x - 1]));
            }
        }
        // right
        if x < maps[y].len() - 1 {
            if maps[y][x + 1] > depth && maps[y][x + 1] < 9 {
                stack.push((x + 1, y, maps[y][x + 1]));
            }
        }
        // down
        if y < maps.len() - 1 {
            if maps[y + 1][x] > depth && maps[y + 1][x] < 9 {
                stack.push((x, y + 1, maps[y + 1][x]));
            }
        }
    }
}

pub fn aoc9_2() {
    let maps = get_input();
    let mut basins: Vec<Basin> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..maps.len() {
        for j in 0..maps[i].len() {
            if visited.contains(&(j, i)) {
                continue;
            }
            if maps[i][j] == 9 {
                visited.insert((j, i));
                continue;
            }
            let mut basin = Basin {
                xs: Vec::new(),
                ys: Vec::new(),
                depths: Vec::new(),
            };
            if is_lowpoint(&maps, j, i) {
                traverse(&maps, &mut basin, &mut visited, j, i);
                basins.push(basin);
            }
        }
    }
    basins.sort_by(|a, b| (*a).xs.len().cmp(&(*b).xs.len()));
    let biggest = basins.pop().unwrap();
    let next_biggest = basins.pop().unwrap();
    let last_biggest = basins.pop().unwrap();
    println!("AOC9.2: {}", biggest.depths.len() * next_biggest.depths.len() * last_biggest.depths.len());
}