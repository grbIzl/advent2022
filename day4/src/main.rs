use std::fs::File;
use std::io::{BufRead, BufReader};

fn prepare_data(input: &str) -> Vec<Vec<i32>> {
    let mut v = Vec::new();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut line: Vec<i32> = line.unwrap().replace("-", ",")
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        // ghetto ordering - ordering tuples would be better (a,b) (c,d) by first elem
        if line[0] > line[2] {
            let a = line[2];
            let b = line[3];
            line[2] = line[0];
            line[0] = a;
            line[3] = line[1];
            line[1] = b;
        }
        v.push(line);
    }
    v
}

fn part_1(v: &Vec<Vec<i32>>) -> usize {
    let mut sum = 0;
    for elem in v {
        // Left contains right
        if elem[0] <= elem[2] && elem[1] >= elem[3] {
            sum += 1;
        }
        // Right contains left
        else if elem[0] >= elem[2] && elem[1] <= elem[3] {
            sum += 1;
        }
    }
    sum
}

fn part_2(v: &Vec<Vec<i32>>) -> usize {
    let mut sum = 0;
    for elem in v {
        if elem[1] >= elem[2] {
            sum += 1
        }
    }
    sum
}

fn main() {
    let vec = prepare_data("input.txt");
    for elem in &vec {
        println!("{:?}", elem)
    }
    let res_1 = part_1(&vec);
    let res_2 = part_2(&vec);
    println!("part1: {} part2: {}", res_1, res_2);
}
