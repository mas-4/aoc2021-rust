use std::fs;
use std::fmt;

// can only be a square
struct Board {
    board: Vec<u32>,
    marked: Vec<bool>,
    size: usize,
    name: i32
}

impl Board {
    fn build(input: &[&str], name: i32) -> Self {
        let size = input.len();
        let marked = vec![false; size * size];
        let input = input.join(" ");
        let board = input
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Board {
            board,
            marked,
            size,
            name,
        }
    }
    fn mark(&mut self, num: u32) {
        for i in 0..(self.size*self.size) {
            if self.board[i] == num {
                self.marked[i] = true;
            }
        }
    }
    fn bingo(&self) -> bool {
        for i in 0..self.size {
            let row = self.marked[i * self.size..(i + 1) * self.size].to_vec();
            let col = vec![
                self.marked[i + (self.size * 0)],
                self.marked[i + (self.size * 1)],
                self.marked[i + (self.size * 2)],
                self.marked[i + (self.size * 3)],
                self.marked[i + (self.size * 4)],
            ];
            if row.iter().all(|x| *x) || col.iter().all(|x| *x) {
                return true;
            }
        }
        false
    }
    fn sum(&self) -> u32 {
        let mut sum = 0;
        for (i, val) in self.board.iter().enumerate() {
            if !self.marked[i] {
                sum += val;
            }
        }
        sum
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Board {}:\n", self.name)?;
        for i in 0..self.size {
            for j in 0..self.size {
                if self.marked[i * self.size + j] {
                    write!(f, " X ")?;
                } else {
                    write!(f, " - ",)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn read_input() -> (Vec<Board>, Vec<u32>) {
    let contents = fs::read_to_string("input/aoc04.txt").expect( "Couldn't open." );
    let input: Vec<&str> = contents
        .lines()
        .filter(|x| x.len() > 0)
        .collect();
    let numbers: Vec<u32> = input[0]
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let boards = input[1..]
        .chunks(5)
        .enumerate()
        .map(|(i, x)| Board::build(x, i as i32))
        .collect::<Vec<Board>>();
    (boards, numbers)
}

pub fn aoc4_1() {
    let (mut boards, nums) = read_input();
    for num in nums {
        for board in boards.iter_mut() {
            board.mark(num);
            if board.bingo() {
                println!("AOC4.1: {}", board.sum() * num);
                return;
            }
        }
    }
}

pub fn aoc4_2() {
    let (mut boards, nums) = read_input();

    for num in nums {
        for i in 0..boards.len() {
            boards[i].mark(num);
        }
        if boards.len() > 1 {
            boards = boards.into_iter().filter(|x| !x.bingo()).collect();
        }
        // println!("{}", boards[0]);
        if boards.len() == 1 && boards[0].bingo() {
            println!("AOC4.2: {}", boards[0].sum() * num);
            return;
        }
    }
}