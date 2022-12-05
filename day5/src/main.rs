use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

// Sorry :D
fn prepare_stacks(part: &str) -> Vec<LinkedList<char>> {
    let mut v = Vec::new();
    if part == "part1" {
        v.push(LinkedList::from(['_']));
        v.push(LinkedList::from(['Z', 'N']));
        v.push(LinkedList::from(['M', 'C', 'D']));
        v.push(LinkedList::from(['P']))
    } else if part == "part2" {
        v.push(LinkedList::from(['_']));
        v.push(LinkedList::from(['D', 'L', 'V', 'T', 'M', 'H', 'F']));
        v.push(LinkedList::from(['H', 'Q', 'G', 'J', 'C', 'T', 'N', 'P']));
        v.push(LinkedList::from(['R', 'S', 'D', 'M', 'P', 'H']));
        v.push(LinkedList::from(['L', 'B', 'V', 'F']));
        v.push(LinkedList::from(['N', 'H', 'G', 'L', 'Q']));
        v.push(LinkedList::from(['W', 'B', 'D', 'G', 'R', 'M', 'P']));
        v.push(LinkedList::from(['G', 'M', 'N', 'R', 'C', 'H', 'L', 'Q']));
        v.push(LinkedList::from(['C', 'L', 'W']));
        v.push(LinkedList::from(['R', 'D', 'L', 'Q', 'J', 'Z', 'M', 'T']));
    }
    v
}

fn prepare_commands(input: &str) -> Vec<(u8, u8, u8)> {
    let mut v = Vec::new();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (count, from, to) = scan_fmt!( &line,  // input string
                            "move {} from {} to {}",     // format
                            u8, u8, u8).unwrap();
        v.push((count, from, to));
    }
    v
}

fn main() {
    // let mut stacks = prepare_stacks("part1");
    // let commands = prepare_commands("test.txt");

    let mut stacks = prepare_stacks("part2");
    let commands = prepare_commands("input.txt");

    // //Part 1
    // for (count, from, to) in commands {
    //     for _ in 0..count {
    //         let elem = stacks[usize::from(from)].pop_back().unwrap();
    //         stacks[usize::from(to)].push_back(elem);
    //     }
    // }

    //Part 2
    for (count, from, to) in commands {
        let mut helper_list = LinkedList::new();
        for _ in 0..count {
            let elem = stacks[usize::from(from)].pop_back().unwrap();
            helper_list.push_back(elem);
        }
        for _ in 0..helper_list.len() {
            let elem = helper_list.pop_back().unwrap();
            stacks[usize::from(to)].push_back(elem);
        }
    }

    for mut stack in stacks {
        let elem = stack.pop_back().unwrap();
        print!("{elem}")
    }
}
