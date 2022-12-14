use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::Range;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn score_slice(trees: &Vec<Vec<u32>>, range1: &Range<usize>, range2: &Range<usize>, tree_height: u32) -> u32 {
    let mut score: u32 = 0;
    for i in range1.start..range1.end {
        for j in range2.start..range2.end {
            score = score + 1;
            if trees[i][j] >= tree_height {
                return score;
            }
        }
    }
    return score;
}

fn score_slice_rev(trees: &Vec<Vec<u32>>, range1: &Range<usize>, range2: &Range<usize>, tree_height: u32) -> u32 {
    let mut score: u32 = 0;
    for i in (range1.start..range1.end).rev() {
        for j in (range2.start..range2.end).rev() {
            score = score + 1;
            if trees[i][j] >= tree_height {
                return score;
            }
        }
    }
    return score;
}

fn check_slice(trees: &Vec<Vec<u32>>, range1: &Range<usize>, range2: &Range<usize>, tree_height: u32) -> bool {
    for i in range1.start..range1.end {
        for j in range2.start..range2.end {
            if trees[i][j] >= tree_height {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    if let Ok(lines) = read_lines("./input2.txt") {
        let mut trees: Vec<Vec<u32>> = Vec::new();
        //let mut high_trees = 0;
        for line in lines {
            if let Ok(data) = line {
                trees.push(data.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>());
            }
        }
        //println!("Trees are {:?}", trees);
        let mut max_prod = 1;
        for vec_index in 1..trees.len() - 1 {
            for tree_index in 1..trees[vec_index].len() - 1 {
                let tree_height = trees[vec_index][tree_index];
                /*let slice1_check = check_slice(&trees, &(vec_index..vec_index+1), &(0..tree_index), tree_height);
                let slice2_check = check_slice(&trees, &(vec_index..vec_index+1), &(tree_index+1..trees[vec_index].len()), tree_height);
                let slice3_check = check_slice(&trees, &(0..vec_index), &(tree_index..tree_index+1), tree_height);
                let slice4_check = check_slice(&trees, &(vec_index+1..trees.len()), &(tree_index..tree_index+1), tree_height);
                if slice1_check || slice2_check || slice3_check || slice4_check {
                    high_trees = high_trees + 1;
                }*/
                let slice1_score = score_slice_rev(&trees, &(vec_index..vec_index+1), &(0..tree_index), tree_height);
                let slice2_score = score_slice(&trees, &(vec_index..vec_index+1), &(tree_index+1..trees[vec_index].len()), tree_height);
                let slice3_score = score_slice_rev(&trees, &(0..vec_index), &(tree_index..tree_index+1), tree_height);
                let slice4_score = score_slice(&trees, &(vec_index+1..trees.len()), &(tree_index..tree_index+1), tree_height);
                let mut prod = 1;
                if slice1_score != 0 {
                    prod = prod * slice1_score;
                }
                if slice2_score != 0 {
                    prod = prod * slice2_score;
                }
                if slice3_score != 0 {
                    prod = prod * slice3_score;
                }
                if slice4_score != 0 {
                    prod = prod * slice4_score;
                }
                if prod > max_prod {
                    max_prod = prod;
                }
            }
        }

        //println!("High trees are {:?}", high_trees);
        //let edges_trees = trees.len() * 2 + trees[1].len() * 2 - 4;
        //println!("Total high trees {}", edges_trees + high_trees);
        println!("Max score is {}", max_prod);
    }
}