use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

fn prepare_commands(input: &str) -> (Vec<LinkedList<char>>, Vec<(u8, u8, u8)>) {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut temp_stacks = Vec::new();
    let mut stacks = Vec::new();
    let mut commands = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() || line.trim().starts_with("1") {
            continue;
        //Commands parsing
        } else if line.trim().starts_with("move") {
            let (count, from, to) = scan_fmt!( &line,  // input string
                            "move {} from {} to {}",     // format
                            u8, u8, u8).unwrap();
            commands.push((count, from, to));
        //Matrix building of the stacks
        } else {
            let mut fixed = line.replace("    ", "_").replace("[", "").replace("]", "");
            fixed.retain(|c| !c.is_whitespace());
            let vec: Vec<char> = fixed.chars().collect();
            temp_stacks.push(vec)
        }
    }
    // Building the stack from the matrix
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
    println!();
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
