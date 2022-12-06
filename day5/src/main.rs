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

fn prepare_commands(input: &str) -> (Vec<LinkedList<char>>, Vec<(u8, u8, u8)>) {
    let mut temp_stacks = Vec::new();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let mut stacks = Vec::new();
    let mut commands = Vec::new();

    for line in reader.lines() {
        // Building the stack
        let line = line.unwrap();
        if line.trim().is_empty() || line.trim().starts_with("1") {
            continue;
        } else if line.trim().starts_with("move") {
            let (count, from, to) = scan_fmt!( &line,  // input string
                            "move {} from {} to {}",     // format
                            u8, u8, u8).unwrap();
            commands.push((count, from, to));
        } else {
            let mut fixed = line.replace("    ", "_").replace("[", "").replace("]", "");
            fixed.retain(|c| !c.is_whitespace());
            let vec: Vec<char> = fixed.chars().collect();
            temp_stacks.push(vec)
        }
    }
    for i in 0..temp_stacks[0].len() {
        let mut stack = LinkedList::new();
        for j in (0..temp_stacks.len()).rev() {
            if temp_stacks[j][i] != '_' {
                stack.push_back(temp_stacks[j][i])
            }
        }
        stacks.push(stack.clone());
    }
    (stacks, commands)
}

fn main() {
    let (mut stacks, commands) = prepare_commands("input.txt");
    let mut stacks_2 = stacks.clone();

    // Part 1
    for (count, from, to) in &commands {
        for _ in 0..*count {
            let elem = stacks[usize::from(from - 1)].pop_back().unwrap();
            stacks[usize::from(to - 1)].push_back(elem);
        }
    }
    println!("Part 1 solution: ");
    for mut stack in stacks {
        let elem = stack.pop_back().unwrap();
        print!("{elem}")
    }
    println!("");
    //Part 2
    for (count, from, to) in &commands {
        let mut helper_list = LinkedList::new();
        for _ in 0..*count {
            let elem = stacks_2[usize::from(from - 1)].pop_back().unwrap();
            helper_list.push_back(elem);
        }
        for _ in 0..helper_list.len() {
            let elem = helper_list.pop_back().unwrap();
            stacks_2[usize::from(to - 1)].push_back(elem);
        }
    }
    println!("Part 2 solution: ");
    for mut stack in stacks_2 {
        let elem = stack.pop_back().unwrap();
        print!("{elem}")
    }
}
