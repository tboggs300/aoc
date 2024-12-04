use std::fs::read_to_string;

fn main() {
    let mut results = Vec::<bool>::new();
    for line in read_to_string("input.txt").expect("Unable to read file").lines() {
        results.push(calc_with_dampener(
                &mut line
                    .split(' ')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
                ));
    };

    println!("Number of safe reports: {}", results.iter().filter(|x| **x).count());
}

fn calc_with_dampener(levels: &mut [i32]) -> bool {
    if calc(levels) { return true; }
    levels.reverse();
    if calc(levels) { return true; }
    false
}

fn calc(levels: &[i32]) -> bool {
    let mut last = 0;
    let mut last_diff = 0;
    let mut res = true;
    let mut chance = true;
    for (idx, level) in levels.iter().enumerate() {
        if idx == 0 {
            last = *level;
        } else {
            if !res { break; };
            let diff = level - last;
            match diff {
                _ if (last_diff > 0 && diff < 0) || (last_diff < 0 && diff > 0) => {
                    res = false;
                },
                _ if diff.abs() < 1 || diff.abs() > 3 => {
                    res = false;
                },
                _ => {},
            };
            if chance && !res {
                res = true;
                chance = false;
            } else {
                last = *level;
                last_diff = diff;
            }
        };
    };
    res
}


