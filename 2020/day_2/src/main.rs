use std::{fs, env};

fn main() {
    let path = env::current_dir().unwrap().to_str().unwrap().to_string();
    let filepath = format!("{}/input.txt", path);
    let num = fs::read_to_string(filepath.clone()).unwrap().lines().filter(|line| {
        let l = line.to_string();
        let mut split = l.split(" ");
        let mut nums = split.next().unwrap().split('-');
        let min = nums.next().unwrap().parse::<usize>().unwrap();
        let max = nums.next().unwrap().parse::<usize>().unwrap();
        let c = split.next().unwrap().chars().next().unwrap();
        let check_string = split.next().unwrap();
        let indices = check_string.match_indices(c).filter(|ci| ci.0 + 1 == min || ci.0 + 1 == max).count();
        indices == 1
    }).count();
    println!("{}", filepath.to_string());
    println!("{}", num);
}
