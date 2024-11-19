use std::fs;

fn main() {
    let mut lines = Vec::new();
    let mut v1: u32 = 0;
    let mut v2: u32 = 0;

    for line in fs::read_to_string("../input.txt").unwrap().lines() {
        lines.push(line.to_string().parse::<u32>().unwrap());
    }

    while let Some(x) = lines.pop() {
        let mut found = false;
        for y in &lines {
            if x + y == 2020 {
               found = true;
               v1 = x;
               v2 = *y;
               break;
           }
        }

       if found {
           break;
       }
    }

    println!("{}", v1 * v2);

}
