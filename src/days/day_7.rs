extern crate evalexpr;
use rayon::prelude::*;

use crate::utils::files::get_file;
use evalexpr::*;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

pub fn run() {
    let calibrations = read_lines("./input/day7-test.txt").expect("Failed to read lines");

    println!("sum: {:?}", calibrations
        .par_iter()
        .map(|(k, n)| process_line(n, k.clone()))
        .sum::<usize>());
}

fn process_line(numbers: &Vec<usize>, key: usize) -> usize {
    let expressions = build_calculations(numbers);

    calculate_expressions(&expressions, key)
}

fn calculate_expressions(expressions: &Vec<String>, key: usize) -> usize {
    if let Some(..) = expressions
        .par_iter()
        .find_any(|e| calculate_expression(e, key) == key) {
        key
    } else {
        0
    }
}

fn calculate_expression(expression: &String, key: usize) -> usize {
    let first = Regex::new(r"^\d+.[+*|].\d+").unwrap();
    if let Some(value) = first.find(expression) {
        let result: String;
        if value.as_str().contains("|") {
            result = eval(value.as_str().replace(" | ", "").as_str()).unwrap().to_string();
        } else {
            result = eval(value.as_str()).unwrap().to_string();
        }
        if result.parse::<usize>().unwrap() > key {
            return 0;
        }
        let new_expression = first.replace(expression, &result);
        calculate_expression(&new_expression.to_string(), key)
    } else {
        expression.parse::<usize>().unwrap()
    }
}

fn build_calculations(numbers: &Vec<usize>) -> Vec<String> {
    let mut expressions: Vec<String> = Vec::new();
    let mut temp_expressions = Vec::new();
    expressions.push(numbers[0].to_string());
    for n in 1..numbers.len() {
        temp_expressions.clear();
        for exp in &expressions {
            temp_expressions.push(format!("{} + {}", exp, numbers[n]));
            temp_expressions.push(format!("{} * {}", exp, numbers[n]));
            temp_expressions.push(format!("{} | {}", exp, numbers[n]));
        }
        expressions = temp_expressions.clone();
    }

    expressions
}


fn read_lines(filename: &str) -> io::Result<HashMap<usize, Vec<usize>>> {
    let file = get_file(filename);
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut calibrations = HashMap::new();
    for line in lines {
        let parts: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
        let key = parts[0].parse::<usize>().unwrap();
        let values: Vec<usize> = parts[1].split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        calibrations.insert(key, values);
    }

    Ok(calibrations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_line_should_return_key_value_when_line_valid() {
        let key = 10;
        let mut calibrations = HashMap::new();
        calibrations.insert(1, vec![2, 3, 2]);

        assert_eq!(process_line(&vec![2, 3, 2], key), 10);
    }

    #[test]
    fn process_line_should_return_zero_when_line_invalid() {
        let key = 10;
        let mut calibrations = HashMap::new();
        calibrations.insert(1, vec![1, 1, 1]);
        assert_eq!(process_line(&vec![1, 1, 1], key), 0);
    }

    #[test]
    fn build_calculations_should_return_correct_number_of_expressions() {
        let two_elements_input = build_calculations(&vec![1, 2]);
        assert_eq!(two_elements_input.len(), 3);
        let three_elements_input = build_calculations(&vec![1, 2, 3]);
        assert_eq!(three_elements_input.len(), 9);
        let four_elements_input = build_calculations(&vec![1, 2, 3, 4]);
        assert_eq!(four_elements_input.len(), 27);
    }

    #[test]
    fn calculate_expression_should_return_correct_result() {
        assert_eq!(calculate_expression(&"1 + 2".to_string(), 3), 3);
        assert_eq!(calculate_expression(&"12".to_string(), 12), 12);
        assert_eq!(calculate_expression(&"1 * 2".to_string(), 2), 2);
        assert_eq!(calculate_expression(&"1 + 2 * 3".to_string(), 9), 9);
        assert_eq!(calculate_expression(&"1 + 2 | 3".to_string(), 33), 33);
        assert_eq!(calculate_expression(&"1 * 2 + 3 * 4".to_string(), 20), 20);
    }

    #[test]
    fn calculate_expressions_should_return_early_when_over_limit() {
        assert_eq!(calculate_expression(&"1 * 2 + 3 * 4".to_string(), 2), 0);
    }
}
