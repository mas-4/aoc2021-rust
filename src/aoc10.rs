fn read_input() -> String {
    include_str!("../input/aoc10.txt").to_string()
}


fn get_opener(c: char) -> char {
    match c {
        '}' => '{',
        ']' => '[',
        ')' => '(',
        '>' => '<',
        _ => panic!("What the heck is this: {}", c),
    }
}


pub fn aoc10_1() {
    let input = read_input();
    let mut score = 0;
    for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            // if character is an opener [<({
            if c == '[' || c == '<' || c == '{' || c == '(' {
                stack.push(c);
            } else {
                let opener = stack.pop().expect("One too many closers");
                if opener != get_opener(c) {
                    score += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!("What the heck is this closer: {}", c),
                    }
                }
            }
        }
    }
    println!("AOC10.1: {}", score);
}

pub fn aoc10_2() {
    let input = read_input();
    let mut scores: Vec<i64> = Vec::new();
    'outer: for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            if c == '[' || c == '<' || c == '{' || c == '(' {
                stack.push(c);
            } else {
                let opener = stack.pop().expect("One too many closers");
                if opener != get_opener(c) {
                    continue 'outer;
                }
            }
        }
        let mut score: i64 = 0;
        while !stack.is_empty() {
            let opener = stack.pop().unwrap();
            score *= 5;
            score += match opener {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("What the heck is this opener: {}", opener),
            }
        }
        scores.push(score);
    }
    scores.sort();
    let score = scores[scores.len() / 2];
    println!("AOC10.2: {}", score);
}