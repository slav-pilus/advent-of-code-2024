use std::fs::File;
use std::io::{self, BufRead};

fn read_lines_from_file(filename: &str) -> io::Result<(Vec<u32>, Vec<u32>)> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            std::process::exit(1);
        }
    };

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    for line in lines {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if let [first, second] = numbers[..] {
            left_list.push(first.parse::<u32>().unwrap());
            right_list.push(second.parse::<u32>().unwrap());
        }
    }

    left_list.sort();
    right_list.sort();

    Ok((left_list, right_list))
}

fn main() {
    let (left_list, right_list) =
        read_lines_from_file("./input/day1.txt").expect("Failed to read lines");
    let distance = get_distance(&left_list, &right_list);
    let similarity = get_total_similarity(&left_list, &right_list);

    println!("distance is: {}", distance);
    println!("similarity score is: {}", similarity);
}

fn get_total_similarity(left_list: &Vec<u32>, right_list: &Vec<u32>) -> i32 {
    let mut similarity_score = 0;
    for l in 0..left_list.len() {
        let count = right_list.iter().filter(|e| **e == left_list[l]).count();
        let similarity = count as i32 * left_list[l] as i32;
        similarity_score += similarity;
    }

    similarity_score
}

fn get_distance(left_list: &Vec<u32>, right_list: &Vec<u32>) -> i32 {
    let mut distance = 0;
    if left_list.len() == right_list.len() {
        for l in 0..left_list.len() {
            let diff = (left_list[l] as i32) - (right_list[l] as i32);
            distance += diff.abs();
        }
    }

    distance
}
