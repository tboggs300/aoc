use std::{fs, env};

fn main() {
    const RADIX: u32 = 10;
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
        let num_of_occ = check_string.chars().filter(|ch| &c == ch).count();
        println!("{} {}: {}-{} = {}", check_string, c, min, max, num_of_occ);

        num_of_occ >= min && num_of_occ <= max
        // let c = arr[1];
        // let check_string = arr[2];
    }).count();
    println!("{}", filepath.to_string());
    println!("{}", num);
}
