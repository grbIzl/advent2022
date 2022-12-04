use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::{HashMap, HashSet};
use itertools::Itertools; // 0.9.0

fn prepare_data_part1(input: &str) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let len = line.len();
        let first_half = &line[..len / 2];
        let second_half = &line[len / 2..];
        let set1: HashSet<char> = first_half.chars().collect();
        let set2: HashSet<char> = second_half.chars().collect();
        let intersect = set1.intersection(&set2).next().unwrap();
        v.push(*intersect)
    }
    v
}

fn prepare_data_part2(input: &str) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    for mut chunk in &reader.lines().chunks(3) {
        let set1: HashSet<char> = chunk.next().unwrap().unwrap().chars().collect();
        let set2: HashSet<char> = chunk.next().unwrap().unwrap().chars().collect();
        let set3: HashSet<char> = chunk.next().unwrap().unwrap().chars().collect();

        // Kill me, multiple set intersection
        let mut common_char_list = Vec::new();
        common_char_list.push(set1);
        common_char_list.push(set2);
        common_char_list.push(set3);
        let common_characters: HashSet<&char> = common_char_list[0]
            .iter()
            .filter(|b| common_char_list[1..].iter().all(|set| set.contains(*b)))
            .collect();
        v.push(**common_characters.iter().next().unwrap())
    }
    v
}

fn create_mapping() -> HashMap<char, usize> {
    let mut map = HashMap::new();
    let input_lower = String::from("abcdefghijklmnopqrstuvwxyz");
    let input_capitalized = input_lower.to_uppercase();
    for (index, char) in (input_lower + &input_capitalized).chars().enumerate() {
        map.insert(char, index + 1);
    }
    map
}

fn main() {
    let data = prepare_data_part1("input.txt");
    let mapping = create_mapping();
    // First part
    let mut sum = 0;
    for elem in data {
        sum += mapping.get(&elem).unwrap();
    }
    println!("Part 1: {sum}");

    // Second part
    let mut sum = 0;
    let data = prepare_data_part2("input.txt");
    for elem in data {
        sum += mapping.get(&elem).unwrap();
    }
    println!("Part 2: {sum}");

}
