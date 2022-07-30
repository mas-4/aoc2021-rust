fn read_input() -> (Vec<Vec<String>>, Vec<Vec<String>>) {  // input output
    let raw = include_str!("../input/aoc08.txt");
    let mut input: Vec<Vec<String>> = Vec::new();
    let mut output: Vec<Vec<String>> = Vec::new();
    for line in raw.lines() {
        let (inp, outp) = line.split_once(" | ").unwrap();
        let inp: Vec<String> = inp
            .split(" ")
            .map(|x| {
                let mut x: Vec<char> = x.chars().collect();
                x.sort_by(|a, b| a.cmp(b));
                String::from_iter(x)
            })
            .collect();
        input.push(inp);
        let outline = outp
            .split(" ")
            .map(|x| {
                let mut x: Vec<char> = x.chars().collect();
                x.sort_by(|a, b| a.cmp(b));
                String::from_iter(x)
            })
            .collect::<Vec<String>>();
        output
            .push(outline);
    }
    (input, output)
}

pub fn aoc8_1() {
    let (_, output) = read_input();
    // flatten output
    let flattened = output
        .iter()
        .flat_map(|x| x.iter())
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let res = flattened
        .iter()
        .fold(0, |acc, x| {
            if matches!(x.len(), 2 | 3 | 4 | 7) {
                acc + 1
            } else {
                acc
            }
        });
    println!("AOC8.1: {}", res);
}


// 1, 4, 7, 8
fn get_starters(input: &Vec<String>) -> (String, String, String, String)
{
    let one_idx = input
        .iter()
        .position(|x| x.len() == 2)
        .unwrap();
    let four_idx = input
        .iter()
        .position(|x| x.len() == 4)
        .unwrap();
    let seven_idx = input
        .iter()
        .position(|x| x.len() == 3)
        .unwrap();
    let eight_idx = input
        .iter()
        .position(|x| x.len() == 7)
        .unwrap();
    (
        input[one_idx].clone(),
        input[four_idx].clone(),
        input[seven_idx].clone(),
        input[eight_idx].clone(),
    )
}

fn get_contains_substr(possibilities: &Vec<String>, substr: &String) -> String {
    for p in possibilities {
        let mut notfound = false;
        for ch in substr.chars() {
            if !p.contains(ch) {
                notfound = true;
                break;
            }
        }
        if !notfound {
            return p.to_string();
        }
    }
    panic!("No substr found: {:?}: {}", possibilities, substr);
}


fn get_doesnt_contain_substr(possibilities: &Vec<String>, substr: &String) -> String {
    for p in possibilities {
        let mut notfound = false;
        for ch in substr.chars() {
            if !p.contains(ch) {
                notfound = true;
                break;
            }
        }
        if notfound {
            return p.to_string();
        }
    }
    panic!("No lack found");
}

fn get_difference(superset: &String, subset: &String) -> char {
    // finds the first char in the superset that is not in the subset
    for ch in superset.chars() {
        if !subset.contains(ch) {
            return ch;
        }
    }
    panic!("No difference found between {} and {}", superset, subset);
}

/*
The board!

  0:      1:      2:      3:      4:
 aaaa            aaaa    aaaa
b    c       c       c       c  b    c
b    c       c       c       c  b    c
                 dddd    dddd    dddd
e    f       f  e            f       f
e    f       f  e            f       f
 gggg            gggg    gggg

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b       b            c  b    c  b    c
b       b            c  b    c  b    c
 dddd    dddd            dddd    dddd
     f  e    f       f  e    f       f
     f  e    f       f  e    f       f
 gggg    gggg            gggg    gggg

 0: abcefg
 1: cf
 2: acdeg
 3: acdfg
 4: bcdf
 5: abdfg
 6: abdefg
 7: acf
 8: abcdefg
 9: abcdfg

 */


fn get_digits(input: &Vec<String>) -> Vec<String> {

    let unique = input
        .clone()
        .iter()
        .fold(Vec::new(), |mut acc, x| {
            if !acc.contains(x) {
                acc.push(x.clone());
            }
            acc
        });
    // known  unique lengths
    let (one, four, seven, eight) = get_starters(&unique);
    // now: 1, 4, 7, 8

    // strings with len five: 2, 3, 5
    let mut five_segs: Vec<String> = unique
        .iter()
        .filter(|x| x.len() == 5)
        .cloned()
        .collect::<Vec<String>>();

    // three is the only (2, 3, 5) that contains 1 (cf)
    let three = get_contains_substr(&five_segs, &one);
    // now: 1, 3, 4, 7, 8

    let idx = five_segs.iter().position(|x| x == &three).unwrap();
    five_segs.remove(idx); // left: (2, 5)

    // strings with len six: 0, 6, 9
    let mut six_segs = unique
        .iter()
        .filter(|x| x.len() == 6)
        .cloned()
        .collect::<Vec<String>>();

    // Between (0, 6, 9) only 6 doesn't contain 1 (cf)
    let six = get_doesnt_contain_substr(&six_segs, &one);
    let idx = six_segs.iter().position(|x| x == &six).unwrap();
    six_segs.remove(idx); // left: 0, 9
    // now: 1, 3, 4, 6, 7, 8

    // c is the only char that is in three but not in six
    let c = get_difference(&three, &six);
    // of (2, 5) only 2 contains c
    let two = get_contains_substr(&five_segs, &c.to_string());
    let five = get_doesnt_contain_substr(&five_segs, &c.to_string());
    // now: 1, 2, 3, 4, 5, 6, 7, 8

    // 6 contains e but 5 doesn't
    let e = get_difference(&six, &five);
    // 0 contains e but 9 doesn't
    let zero = get_contains_substr(&six_segs, &e.to_string());
    let nine = get_doesnt_contain_substr(&six_segs, &e.to_string());

    // now: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
    // All your number are belong to us
    vec![
        zero, one, two, three, four, five, six, seven, eight, nine,
    ]
}

pub fn aoc8_2() {
    let (full_input, full_output) = read_input();
    let mut sum = 0;
    for i in 0..full_input.len() {
        let input = full_input[i].clone();
        let output = full_output[i].clone();
        let digits = get_digits(&input);
        for (i, dig) in output.iter().enumerate() {
            let magnitude = output.len() - i - 1;
            let num = match digits
                .iter()
                .position(|x| x == &dig.to_string()) {
                    Some(x) => x,
                    None => {
                        println!("No digit found for {}", dig);
                        panic!("No digit found")
                    },
                };
            // we don't have to actually construct the entire number to add it to the sum
            sum += num * 10usize.pow(magnitude as u32);
        }
    }

    println!("AOC8.2: {}", sum);
}