use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use itertools::Itertools; // 0.9.0

fn prepare_data(input: &str) -> Vec<(char, char)> {
    let mut v: Vec<(char, char)> = Vec::new();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let chars: (char, char) = line.chars().filter(|c| !c.is_whitespace()).collect_tuple().unwrap();
        v.push(chars)
    }
    v
}

fn calculate_scores_1(v: &Vec<(char, char)>) -> usize {
    let mut sum = 0;
    for elem in v {
        match elem {
            //R1, P2, S3, W6, T3, L0
            ('A', 'X') => sum += 1 + 3,
            ('A', 'Y') => sum += 2 + 6,
            ('A', 'Z') => sum += 3 + 0,
            ('B', 'X') => sum += 1 + 0,
            ('B', 'Y') => sum += 2 + 3,
            ('B', 'Z') => sum += 3 + 6,
            ('C', 'X') => sum += 1 + 6,
            ('C', 'Y') => sum += 2 + 0,
            ('C', 'Z') => sum += 3 + 3,
            _ => panic!("Invalid pairs: {}, {}", elem.0, elem.1)
        }
    }
    sum
}

fn calculate_scores_2(v: &Vec<(char, char)>) -> usize {
    let mut sum = 0;
    for elem in v {
        match elem {
            //R1, P2, S3, X-Lose, Y-Tie, Z-Win
            ('A', 'X') => sum += 0 + 3,
            ('A', 'Y') => sum += 3 + 1,
            ('A', 'Z') => sum += 6 + 2,
            ('B', 'X') => sum += 0 + 1,
            ('B', 'Y') => sum += 3 + 2,
            ('B', 'Z') => sum += 6 + 3,
            ('C', 'X') => sum += 0 + 2,
            ('C', 'Y') => sum += 3 + 3,
            ('C', 'Z') => sum += 6 + 1,
            _ => panic!("Invalid pairs: {}, {}", elem.0, elem.1)
        }
    }
    sum
}

fn main() {
    let data = prepare_data("input.txt");
    let result1 = calculate_scores_1(&data);
    let result2 = calculate_scores_2(&data);

    println!("Part1: {} Part2: {}", result1, result2)
}
