use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

const MARKER_LEN: usize = 14;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn all_chars_different(chars: &Vec<char>) -> bool {
    let mut chars_set: HashSet<&char> = HashSet::new();
    for c in chars {
        if !chars_set.insert(c) {
            return false;
        }
    }
    return true;
}
fn main() {
    if let Ok(lines) = read_lines("./input2.txt") {
        for line in lines {
            if let Ok(data) = line {
                for (index, _) in data.chars().enumerate() {
                    if index < MARKER_LEN {
                        continue;
                    }

                    let mut char_vec = Vec::new();
                    for i in 1..=MARKER_LEN {
                        char_vec.push(data.chars().nth(index - i).unwrap());
                    }

                    if all_chars_different(&char_vec) {
                        println!("Index is {}", index);
                        break;
                    }
                }
            }
        }
    }
}
