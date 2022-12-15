use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn cells_adjacent(tail: &(i32, i32), head: &(i32, i32)) -> bool {
    return (tail.0 - head.0).abs() <= 1 && (tail.1 - head.1).abs() <= 1;
}

fn update_tail_coord(tail: &mut (i32, i32), head: &(i32, i32)) {
    if head.0 - tail.0 == 2 && head.1 - tail.1 == 0 {
        // b
        tail.0 = tail.0 + 1;
    } else if head.0 - tail.0 == -2 && head.1 - tail.1 == 0 {
        // f
        tail.0 = tail.0 - 1;
    } else if head.0 - tail.0 == 0 && head.1 - tail.1 == 2 {
        // h
        tail.1 = tail.1 + 1;
    } else if head.0 - tail.0 == 0 && head.1 - tail.1 == -2 {
        // d
        tail.1 = tail.1 - 1;
    } else if head.0 - tail.0 == 2 && head.1 - tail.1 == 1 {
        // a
        tail.0 = tail.0 + 1;
        tail.1 = tail.1 + 1;
    } else if head.0 - tail.0 == 1 && head.1 - tail.1 == 2 {
        // a
        tail.0 = tail.0 + 1;
        tail.1 = tail.1 + 1;
    } else if head.0 - tail.0 == 2 && head.1 - tail.1 == 2 {
        // a
        tail.0 = tail.0 + 1;
        tail.1 = tail.1 + 1;

    } else if head.0 - tail.0 == -2 && head.1 - tail.1 == 1 {
        // g
        tail.0 = tail.0 - 1;
        tail.1 = tail.1 + 1;
    } else if head.0 - tail.0 == -1 && head.1 - tail.1 == 2 {
        // g
        tail.0 = tail.0 - 1;
        tail.1 = tail.1 + 1;
    } else if head.0 - tail.0 == -2 && head.1 - tail.1 == 2 {
        // g
        tail.0 = tail.0 - 1;
        tail.1 = tail.1 + 1;
    } else if head.0 - tail.0 == 2 && head.1 - tail.1 == -1 {
        // c
        tail.0 = tail.0 + 1;
        tail.1 = tail.1 - 1;
    } else if head.0 - tail.0 == 1 && head.1 - tail.1 == -2 {
        // c
        tail.0 = tail.0 + 1;
        tail.1 = tail.1 - 1;
    } else if head.0 - tail.0 == 2 && head.1 - tail.1 == -2 {
        // c
        tail.0 = tail.0 + 1;
        tail.1 = tail.1 - 1;
    } else if head.0 - tail.0 == -1 && head.1 - tail.1 == -2 {
        // e
        tail.1 = tail.1 - 1;
        tail.0 = tail.0 - 1;
    } else if head.0 - tail.0 == -2 && head.1 - tail.1 == -1 {
        // e
        tail.1 = tail.1 - 1;
        tail.0 = tail.0 - 1;
    } else if head.0 - tail.0 == -2 && head.1 - tail.1 == -2 {
        // e
        tail.1 = tail.1 - 1;
        tail.0 = tail.0 - 1;
    }
}

fn main() {
    let knots_size = 10;
    if let Ok(lines) = read_lines("./input2.txt") {
        let mut knots: Vec<(i32, i32)> = vec![(0, 0); knots_size];
        let mut visited_cells: HashSet<(i32, i32)> = HashSet::new();
        visited_cells.insert(knots[knots_size-1]);
        for line in lines {
            if let Ok(data) = line {
                let (command, argument) = data.split_at(1);
                let moves: i32 = argument.trim().parse().unwrap();
                for _ in 0..moves {
                    match command {
                        "R" => {
                            knots.get_mut(0).unwrap().0 = knots[0].0 + 1;
                        }
                        "L" => {
                            knots.get_mut(0).unwrap().0 = knots[0].0 - 1;
                        }
                        "U" => {
                            knots.get_mut(0).unwrap().1 = knots[0].1 + 1;
                        }
                        "D" => {
                            knots.get_mut(0).unwrap().1 = knots[0].1 - 1;
                        }
                        _ => {
                            println!("Unknown command");
                        }
                    }

                    for i in 1..knots_size {
                        let head = knots[i-1];
                        if !cells_adjacent(&knots[i], &head) {
                            update_tail_coord(knots.get_mut(i).unwrap(), &head);
                        }
                    }
                    visited_cells.insert(knots[knots_size-1]);
                }
            }

        }

        println!("Visited cells count is {}", visited_cells.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent() {
        assert_eq!(cells_adjacent(&(0, 0), &(0, 1)), true);
        assert_eq!(cells_adjacent(&(0, 0), &(1, 0)), true);
        assert_eq!(cells_adjacent(&(0, 0), &(1, 1)), true);
        assert_eq!(cells_adjacent(&(0, 0), &(1, -1)), true);
        assert_eq!(cells_adjacent(&(0, 0), &(0, -1)), true);
        assert_eq!(cells_adjacent(&(0, 0), &(-1, -1)), true);
        assert_eq!(cells_adjacent(&(0, 0), &(-1, 0)), true);
        assert_eq!(cells_adjacent(&(0, 0), &(-1, 1)), true);


        assert_eq!(cells_adjacent(&(0, 0), &(0, 2)), false);
        assert_eq!(cells_adjacent(&(0, 0), &(1, 2)), false);
        assert_eq!(cells_adjacent(&(0, 0), &(2, 1)), false);
        assert_eq!(cells_adjacent(&(0, 0), &(2, 0)), false);
        assert_eq!(cells_adjacent(&(0, 0), &(2, -1)), false);
        assert_eq!(cells_adjacent(&(0, 0), &(1, -2)), false);
        assert_eq!(cells_adjacent(&(0, 0), &(0, -2)), false);
        assert_eq!(cells_adjacent(&(0, 0), &(-1, -2)), false);
        assert_eq!(cells_adjacent(&(0, 0), &(-2, -1)), false);
        assert_eq!(cells_adjacent(&(0, 0), &(-2, 0)), false);
        assert_eq!(cells_adjacent(&(0, 0), &(-2, 1)), false);
        assert_eq!(cells_adjacent(&(0, 0), &(-1, 2)), false);
    }

    #[test]
    fn test_movement() {
        let mut tail = (0, 0);
        let head = (0, 2);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (0, 1));

        let mut tail = (0, 0);
        let head = (1, 2);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (1, 1));

        let mut tail = (0, 0);
        let head = (2, 2);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (1, 1));

        let mut tail = (0, 0);
        let head = (2, 1);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (1, 1));

        let mut tail = (0, 0);
        let head = (2, 0);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (1, 0));

        let mut tail = (0, 0);
        let head = (2, -1);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (1, -1));

        let mut tail = (0, 0);
        let head = (2, -2);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (1, -1));

        let mut tail = (0, 0);
        let head = (1, -2);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (1, -1));

        let mut tail = (0, 0);
        let head = (0, -2);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (0, -1));

        let mut tail = (0, 0);
        let head = (-1, -2);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (-1, -1));

        let mut tail = (0, 0);
        let head = (-2, -2);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (-1, -1));

        let mut tail = (0, 0);
        let head = (-2, -1);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (-1, -1));

        let mut tail = (0, 0);
        let head = (-2, 0);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (-1, 0));

        let mut tail = (0, 0);
        let head = (-2, 1);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (-1, 1));

        let mut tail = (0, 0);
        let head = (-2, 2);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (-1, 1));

        let mut tail = (0, 0);
        let head = (-1, 2);
        update_tail_coord(&mut tail, &head);
        assert_eq!(tail, (-1, 1));
    }
}