use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::Chars;

use bounded_vec_deque::BoundedVecDeque;
use itertools::Itertools;

fn prepare_commands(input: &str) -> String {
    let mut file = File::open(input).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let input = prepare_commands("input.txt");
    // const SOLUTION: usize = 4;
    const SOLUTION: usize = 14;

    let mut deque: BoundedVecDeque<char> = BoundedVecDeque::with_capacity(SOLUTION, SOLUTION);

    for (index, c) in input.chars().enumerate() {
        if deque.len() == SOLUTION {
            let set = deque.clone().into_iter()
                .collect::<HashSet<char>>();
            if set.len() == SOLUTION {
                println!("The solution is: {}", index);
                break;
            }
        }
        deque.push_back(c);
    }
}
