use std::fs;

fn main() {
    let mut lines = Vec::new();
    let mut v1: u32 = 0;
    let mut v2: u32 = 0;
    let mut v3: u32 = 0;

    for line in fs::read_to_string("../input.txt").unwrap().lines() {
        lines.push(line.to_string().parse::<u32>().unwrap());
    }

    while let Some(x) = lines.pop() {
        let mut found = false;
        for y in &lines {
            for z in &lines {
                if y == z {
                    continue;
                }
                if x + y + z == 2020 {
                    found = true;
                    v1 = x;
                    v2 = *y;
                    v3 = *z;
                    break;
                }

            }
        }

       if found {
           break;
       }
    }

    println!("{} {} {}", v1, v2, v3);
    println!("{}", v1 * v2 * v3);

}
