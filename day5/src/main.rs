use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, VecDeque};
use lazy_static::lazy_static;
use regex::Regex;

type BucketType = HashMap<i8, VecDeque<String>>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_data(data: &String, buckets: &mut BucketType) {
    let mut data_str = data.clone();
    let mut counter = 1;
    while data_str.len() > 4 {
        let part: String = data_str.drain(..4).collect();
        let part = part.replace(" ", "");
        if !part.is_empty() {
            buckets.entry(counter).and_modify(|vec| vec.push_back(part.clone())).or_insert(VecDeque::from(vec![part]));
        }
        counter = counter + 1;
    }

    let data_str = data_str.replace(" ", "");
    if !data_str.is_empty() {
        buckets.entry(counter).and_modify(|vec| vec.push_back(data_str.clone())).or_insert(VecDeque::from(vec![data_str]));
    }
}

fn make_move(data: &String, buckets: &mut BucketType) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    let caps = RE.captures(data).unwrap();
    let items_to_move: i8 = caps[1].parse().unwrap();
    let bucket_src: i8 = caps[2].parse().unwrap();
    let bucket_dst: i8 = caps[3].parse().unwrap();
    let mut items_to_add: VecDeque<String> = VecDeque::new();
    for _ in 0..items_to_move {
        let mut item = None;
        if let Some(vec_src) = buckets.get_mut(&bucket_src) {
            item = vec_src.pop_front();
        }
        if let Some(item) = item {
            items_to_add.push_front(item);
        }
    }

    for item in items_to_add {
        if let Some(vec_dst) = buckets.get_mut(&bucket_dst) {
            vec_dst.push_front(item);
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input2.txt") {
        let mut buckets: BucketType = HashMap::new();
        let mut moves = false;
        for line in lines {
            if let Ok(data) = line {
                if data.is_empty() {
                    moves = true;
                    continue;
                }
                if moves {
                    make_move(&data, &mut buckets);
                } else {
                    parse_data(&data, &mut buckets);
                }
            }
        }
        //println!("{:?}", buckets);
        let mut results: Vec<(i8, &String)> = buckets.iter().map(|(key, vec)| (*key, vec.iter().nth(0).unwrap())).collect();
        results.sort_by(|a, b| b.0.cmp(&a.0).reverse());
        println!("Results {:?}", results);
    }
}