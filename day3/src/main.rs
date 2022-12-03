use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/*fn main() {
    if let Ok(lines) = read_lines("./input2.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(ruicksack) = line {
                let (first, second) = ruicksack.split_at(ruicksack.len() / 2);
                let first_set: HashSet<char> = first.chars().collect();
                let second_set: HashSet<char> = second.chars().collect();
                let intersection = first_set.intersection(&second_set).next().unwrap();
                let intersection_code = *intersection as u32;
                let intersection_code_adjusted = match intersection.is_lowercase() {
                    true => intersection_code - 96,
                    false => intersection_code - 38,
                };
                sum = sum + intersection_code_adjusted;
            }
        }
        println!("Sum {}", sum);
    }
}*/

fn get_intersection_code(first : &HashSet<char>, second: &HashSet<char>, third: &HashSet<char>) -> u32 {
    let intersection1 = first.intersection(second).collect::<HashSet<&char>>();
    let intersection2 = second.intersection(third).collect::<HashSet<&char>>();
    let intersection3 = intersection1.intersection(&intersection2).next().unwrap();
    let intersection_code = **intersection3 as u32;
    let intersection_code_adjusted = match intersection3.is_lowercase() {
        true => intersection_code - 96,
        false => intersection_code - 38,
    };
    intersection_code_adjusted
}

fn main() {
    if let Ok(lines) = read_lines("./input2.txt") {
        let mut sum = 0;
        let mut first_elf: HashSet<char> = HashSet::new();
        let mut second_elf: HashSet<char> = HashSet::new();
        let mut third_elf: HashSet<char> = HashSet::new();
        for (index, line) in lines.enumerate() {
            if let Ok(ruicksack) = line {
                if index % 3 == 0 && !first_elf.is_empty() {
                    sum = sum + get_intersection_code(&first_elf, &second_elf, &third_elf);
                }
                if index % 3 == 0 {
                    first_elf = ruicksack.chars().collect();
                } else if index % 3 == 1 {
                    second_elf = ruicksack.chars().collect();
                } else if index % 3 == 2 {
                    third_elf = ruicksack.chars().collect();
                }
            }
        }
        sum = sum + get_intersection_code(&first_elf, &second_elf, &third_elf);
        println!("Sum {}", sum);
    }
}