use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./input2.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(elves) = line {
                let v: Vec<&str> = elves.split(',').collect();
                let sets: Vec<HashSet<i32>> = v.into_iter().map(|elf| {
                    let parts: Vec<&str> = elf.split('-').collect();
                    assert_eq!(parts.len(), 2);
                    let mut begin: i32 = parts[0].parse().unwrap();
                    let end: i32 = parts[1].parse().unwrap();
                    let mut set: HashSet<i32> = HashSet::new();
                    while begin <= end {
                        set.insert(begin);
                        begin = begin + 1;
                    }
                    set
                }).collect();
                assert_eq!(sets.len(), 2);
                let first = &sets[0];
                let second = &sets[1];
                /*if first.is_subset(&second) || first.is_superset(&second) {
                    sum = sum + 1;
                }*/
                if !first.intersection(&second).collect::<HashSet<_>>().is_empty() {
                    sum = sum + 1;
                }
            }
        }
        println!("Sum {}", sum);
    }
}