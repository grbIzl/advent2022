use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn part_1(input: &str){
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}",line)
    }
}


fn main() {
    let res = part_1("test.txt");
}
