use std::fs;

#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    points: Vec<(i32, i32)>,
}

impl Line {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        let mut points = Vec::new();
        let mut x = x1;
        let mut y = y1;
        while x != x2 || y != y2 {
            points.push((x, y));
            if x < x2 {
                x += 1;
            } else if x > x2 {
                x -= 1;
            }
            if y < y2 {
                y += 1;
            } else if y > y2 {
                y -= 1;
            }
        }
        points.push((x, y));
        Line {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
            points: points,
        }
    }
}

struct Grid {
    points: Vec<Vec<i32>>,
}

impl Grid {
    fn mark_line(&mut self, line: &Line, use_diagonal: bool) {
        if !use_diagonal && line.x1 != line.x2 && line.y1 != line.y2 {
            return;  // not a horizontal or vertical line
        }
        for point in &line.points {
            let (x, y) = point;
            if *x + 1 > self.points.len() as i32 {
                self.points.resize(*x as usize + 1, Vec::new());
            }
            if *y + 1 > self.points[*x as usize].len() as i32 {
                self.points[*x as usize].resize(*y as usize + 1, 0);
            }
            self.points[*x as usize][*y as usize] += 1;
        }
    }
}


fn read_input() -> Vec<Line> {
    let input = fs::read_to_string("input/aoc5.txt")
        .expect("Something went wrong reading the file");
    let mut lines = Vec::new();
    for line in input.lines() {
        // split on ->
        let parts: Vec<&str> = line.split(" -> ").collect();
        // split on ,
        let p1: Vec<&str> = parts[0].split(",").collect();
        let p2: Vec<&str> = parts[1].split(",").collect();
        let line = Line::new(
            p1[0].parse::<i32>().unwrap(),
            p1[1].parse::<i32>().unwrap(),
            p2[0].parse::<i32>().unwrap(),
            p2[1].parse::<i32>().unwrap(),
        );
        lines.push(line);
    }
    lines
}


pub fn aoc5_1() {
    let lines = read_input();
    let mut grid = Grid {
        points: Vec::new(),
    };
    for line in lines {
        grid.mark_line(&line, false);
    }
    let mut count = 0;
    for v in &grid.points {
        for i in v {
            if *i > 1 {
                count += 1;
            }
        }
    }
    println!("AOC5.1: {}", count);
}

pub fn aoc5_2() {
    let lines = read_input();
    let mut grid = Grid {
        points: Vec::new(),
    };
    for line in lines {
        grid.mark_line(&line, true);
    }
    let mut count = 0;
    for v in &grid.points {
        for i in v {
            if *i > 1 {
                count += 1;
            }
        }
    }
    println!("AOC5.2: {}", count);
}
