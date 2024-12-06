use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let p2_reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut p1_total = 0;
    let mut p2_total = 0;
    let mut enabled = true;

    re.find_iter(include_str!("../input.txt"))
        .map(|c| c.as_str())
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            line
                .strip_prefix("mul(").unwrap()
                .strip_suffix(")").unwrap()
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        }).for_each(|mul| p1_total += mul[0] * mul[1]);

    let p2_lines = p2_reg.find_iter(include_str!("../input.txt"))
        .map(|c| c.as_str())
        .collect::<Vec<&str>>();

    for line in p2_lines.iter() {
        match *line {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let mul = line.to_string()
                    .strip_prefix("mul(").unwrap()
                    .strip_suffix(")").unwrap()
                    .split(",")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                if enabled {
                    p2_total += mul[0] * mul[1];
                }

            }
        };
    }

    println!("Part 1: {}", p1_total);
    println!("Part 2: {}", p2_total);


}
