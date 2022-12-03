use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::str::Chars;

fn prepare_data(input: &str) -> Vec<char>{
    let mut v: Vec<char> = Vec::new();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let len = line.len();
        let first_half = &line[..len/2];
        let second_half = &line[len/2..];
        let set1: HashSet<char> = first_half.chars().collect();
        let set2: HashSet<char> = second_half.chars().collect();
        let intersect = set1.intersection(&set2).next().unwrap();
        v.push(*intersect)
    }
    v
}

fn main() {
    let data = prepare_data("test.txt");
    println!("{data:?}")
}
