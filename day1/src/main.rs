use std::fs::File;
use std::io::{self, prelude::*, BufReader};


// fn day_1_2_ugly(v: &mut Vec<i32>, input: &str) -> &mut Vec<i32>{
fn day_1_2_ugly<'a>(v: &'a mut Vec<i32>, input: &'a str) -> &'a mut Vec<i32> {
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
    let mut v: Vec<i32> = Vec::new();
    let res = day_1_2_ugly(&mut v, "input.txt");
    // let day_1 = day_1_ugly("test.txt");
    println!("Solution 1: {}", res[0]);
    println!("Solution 2: {}", res[0] + res[1] + res[2]);
}
