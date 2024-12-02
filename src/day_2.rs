use std::io;
use std::io::BufRead;
use crate::util::get_file;

pub(crate) fn day_2() {
    println!("day 2");
    let mut safe_report_count = 0;
    let mut safe_report_with_dampener_count = 0;
    let input = read_lines_to_int_vector("./input/day2.txt").expect("Failed to read lines");

    for i in 0..input.len() {
        let safe_report = is_safe_report(&input[i]);
        let safe_report_with_dampener = is_safe_report_with_dampener(&input[i]);
        if safe_report {
            safe_report_count += 1;
        }

        if safe_report_with_dampener {
            safe_report_with_dampener_count += 1;
        }
    }

    println!("safe report count is: {}", safe_report_count);
    println!("safe report with dampener count is: {}", safe_report_with_dampener_count);
}

fn is_safe_report_with_dampener(report: &Vec<i32>) -> bool {
    let is_safe = is_safe_report(report);
    if is_safe {
        return true;
    } else {
        for i in 0..report.len() {
            let mut curren_report = report.clone();
            curren_report.remove(i);

            if is_safe_report(&curren_report) {
                return true;
            }
        }
    }

    false
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    let is_increasing = report.windows(2).all(|x| x[0] < x[1]);
    let is_decreasing = report.windows(2).all(|x| x[0] > x[1]);
    let mut prev_num = report[0];

    for i in 0..report.len() {
        if (prev_num - report[i]).abs() > 3 {
            return false;
        }
        prev_num = report[i];
    }

    is_decreasing || is_increasing
}

fn read_lines_to_int_vector(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = get_file(filename);

    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let levels = lines.iter().map(|l| l.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect()).collect();

    Ok(levels)
}