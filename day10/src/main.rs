use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./input3.txt") {
        let mut signals: Vec<i32> = vec![1];
        let mut cycle_index = 0;
        for line in lines {
            if let Ok(data) = line {
                let parts: Vec<&str> = data.split(' ').collect();
                let command: String = String::from(parts[0]);
                let mut argument: i32 = 0;
                if parts.len() > 1 {
                    argument = parts[1].parse().unwrap();
                }
                if command == "noop" {
                    let start_register = signals[cycle_index];
                    signals.push(start_register);
                    cycle_index = cycle_index + 1;
                } else if command == "addx" {
                    let start_register = signals[cycle_index];
                    let end_register = start_register + argument;
                    signals.push(start_register);
                    signals.push(end_register);
                    cycle_index = cycle_index + 2;
                }
            }
        }
        /*let mut accum = 0;
        for i in 0..100 {
            let signals_index = 20 + i * 40;
            if signals.len() < signals_index {
                break;
            }
            let signal_strength: i32 = (signals_index as i32) * signals[signals_index - 1];
            accum = accum + signal_strength;
        }

        println!("Sum is {}", accum);*/
        let mut output: String = String::new();
        for index in 0..signals.len() {
            let screen_pos = index % 40;
            if screen_pos == 0 {
                output.push_str("\n");
            }
            if ((screen_pos as i32)- signals[index]).abs() < 2 {
                output.push('#');
            } else {
                output.push('.');
            }
        }

        println!("{}", output);
    }
}

