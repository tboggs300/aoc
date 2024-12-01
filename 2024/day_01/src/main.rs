use std::fs::read_to_string;

fn main() {
    let mut nums1 = Vec::<i32>::new();
    let mut nums2 = Vec::<i32>::new();

    read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .for_each(|s| {
            if !s.is_empty() {
                let iter: Vec<i32> = s.split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<i32>().expect("unable to parse number"))
                    .collect::<Vec<i32>>();
                nums1.push(iter[0]);
                nums2.push(iter[1]);
            }
        });

    nums1.sort();
    nums2.sort();
    let mut p1_total = 0;
    let mut p2_total = 0;

    // part 1
    for (i, n1) in nums1.iter().enumerate() {
       let diff = n1 - nums2[i];
       p1_total += diff.abs();
    }


    // part 2
    for num in nums1 {
        let count = nums2.iter().filter(|&n| *n == num).count();
        p2_total += num * count as i32;
    }


    println!("Part 1: {}", p1_total);
    println!("Part 2: {}", p2_total);


}
