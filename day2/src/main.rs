use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let conversions: HashMap<&str, &str> = HashMap::from([
        ("A X", "A Z"),
        ("A Y", "A X"),
        ("A Z", "A Y"),
        ("B X", "B X"),
        ("B Y", "B Y"),
        ("B Z", "B Z"),
        ("C X", "C Y"),
        ("C Y", "C Z"),
        ("C Z", "C X"),
    ]);

    let results: HashMap<&str, i32> = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);
    if let Ok(lines) = read_lines("./input2.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(game) = line {
                if !game.is_empty() {
                    let final_index = conversions.get(game.as_str()).unwrap();
                    sum = sum + results.get(final_index).unwrap();
                }
            }
        }
        println!("Sum {}", sum);
    }
}
