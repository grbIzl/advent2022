use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn part_1_2(input: &str) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            // println!("SUM: {}", sum);
            // println!("-----------------------------------------------");
            v.push(sum);
            sum = 0;
        } else {
            // println!("sum: {}, elem: {}", sum, line.trim().parse::<i32>().unwrap());
            sum += line.trim().parse::<i32>().unwrap();
        }
    }
    v.sort();
    v.reverse();
    v
}


fn main() {
    let res = day_1_2_ugly("input.txt");
    println!("Solution 1: {}", res[0]);
    println!("Solution 2: {}", res[0] + res[1] + res[2]);
}
