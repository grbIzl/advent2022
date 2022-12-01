use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./input2.txt") {
        let mut current_elf_calories = 0;
        let mut elves: Vec<i32> = Vec::new();
        for line in lines {
            if let Ok(calorie) = line {
                if calorie.is_empty() {
                    println!("Current elf calories: {}", current_elf_calories);
                    elves.push(current_elf_calories);
                    current_elf_calories = 0;
                } else {
                    let calories: i32 = calorie.parse().unwrap();
                    current_elf_calories = current_elf_calories + calories;
                }
            }
        }

        if current_elf_calories > 0 {
            println!("Current elf calories: {}", current_elf_calories);
            elves.push(current_elf_calories);
        }

        elves.sort_unstable();
        let sum: i32 = elves.iter().rev().take(3).sum();

        println!("Sum {}", sum);
    }
}
