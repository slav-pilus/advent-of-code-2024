use crate::util::get_file;
use regex::Regex;
use std::io;
use std::io::BufRead;

pub(crate) fn day_3() {
    part_1();
    part_2();
}

fn part_2() {
    let lines = read_lines_vector("./input/day3-1-test.txt").expect("Failed to read lines");
    let joined_line = lines.join("");
    let instructions = split_with_conditionals(joined_line);
    let mut pairs = Vec::new();
    let mut sum = 0;
    for instruction in instructions {
        pairs.extend(find_multiplications(&instruction));
    }

    for pair in pairs {
        sum += pair.0 * pair.1;
    }

    println!("sum: {}", sum);
}

fn split_with_conditionals(line: String) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();
    let dos: Vec<String> = line.split("do()").map(|s| s.to_string()).collect();
    for l in dos {
        let parts: Vec<String> = l.split("don't()").map(|s| s.to_string()).collect();
        instructions.push(parts[0].to_string());
    }

    instructions
}

fn part_1() {
    let lines = read_lines_vector("./input/day3-test.txt").expect("Failed to read lines");
    let mut pairs = Vec::new();
    let mut sum = 0;

    for line in lines {
        pairs.extend(find_multiplications(&line));
    }

    for pair in pairs {
        sum += pair.0 * pair.1;
    }

    println!("sum is: {}", sum);
}

fn find_multiplications(line: &String) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(line)
        .map(|cap| {
            (
                cap.get(1).unwrap().as_str().parse().unwrap(),
                cap.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect()
}

fn read_lines_vector(filename: &str) -> io::Result<Vec<String>> {
    let file = get_file(filename);
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    Ok(lines)
}
