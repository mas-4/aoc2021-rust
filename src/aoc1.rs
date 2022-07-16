use std::fs;

pub fn aoc1() {
    let contents = fs::read_to_string("input/aoc1.txt").expect( "Couldn't open." );
    let numbers = contents
        .trim()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut increased = 0;
    let mut last = numbers[0];
    for i in numbers[1..].iter() {
        if *i > last {
            increased += 1;
        }
        last = *i;
    }
    println!("{}", increased);
}
