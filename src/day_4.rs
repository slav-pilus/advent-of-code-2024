use std::io;
use std::io::BufRead;
use crate::util::get_file;

pub(crate) fn day_4() {
    let characters = load_puzzle("./input/day4.txt").expect("Failed to read lines");

    let mut xmas_counter = 0;
    let mut x_mas_counter = 0;
    for i in 0..characters.len() {
        for j in 0..characters[i].len() {
            let xmas_found = part_1(&characters, i, j);
            let x_mas_found = part_2(&characters, i, j);
            xmas_counter += xmas_found;
            x_mas_counter += x_mas_found;
        }
    }

    println!("xmas count is: {}", xmas_counter);
    println!("x-mas count is: {}", x_mas_counter);
}
fn part_2(chars: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if chars[i][j] != 'A' || !(i as i32 > 0 && j as i32 > 0 && i + 1 < chars.len() && j + 1 < chars[i].len()) {
        return 0;
    }
    let mut counter = 0;

    counter += if chars[i - 1][j - 1] == 'M' && chars[i + 1][j + 1] == 'S' && chars[i - 1][j + 1] == 'S' && chars[i + 1][j - 1] == 'M' { 1 } else { 0 };
    counter += if chars[i - 1][j - 1] == 'M' && chars[i + 1][j + 1] == 'S' && chars[i - 1][j + 1] == 'M' && chars[i + 1][j - 1] == 'S' { 1 } else { 0 };
    counter += if chars[i - 1][j - 1] == 'S' && chars[i + 1][j + 1] == 'M' && chars[i - 1][j + 1] == 'S' && chars[i + 1][j - 1] == 'M' { 1 } else { 0 };
    counter += if chars[i - 1][j - 1] == 'S' && chars[i + 1][j + 1] == 'M' && chars[i - 1][j + 1] == 'M' && chars[i + 1][j - 1] == 'S' { 1 } else { 0 };

    counter
}

fn part_1(chars: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if chars[i][j] != 'X' { return 0 }

    let mut counter = 0;

    // check to right
    counter += if j + 3 < chars[i].len() {
        if chars[i][j + 1] == 'M' && chars[i][j + 2] == 'A' && chars[i][j + 3] == 'S' { 1 } else { 0 }
    } else { 0 };

    // check to left
    counter += if j as i32 - 3 >= 0 {
        if chars[i][j - 1] == 'M' && chars[i][j - 2] == 'A' && chars[i][j - 3] == 'S' { 1 } else { 0 }
    } else { 0 };

    // check above
    counter += if i as i32 - 3 >= 0 {
        if chars[i - 1][j] == 'M' && chars[i - 2][j] == 'A' && chars[i - 3][j] == 'S' { 1 } else { 0 }
    } else { 0 };

    // check below
    counter += if i + 3 < chars.len() {
        if chars[i + 1][j] == 'M' && chars[i + 2][j] == 'A' && chars[i + 3][j] == 'S' { 1 } else { 0 }
    } else { 0 };

    // check bottom left
    counter += if i as i32 - 3 >= 0 && j as i32 - 3 >= 0 {
        if chars[i - 1][j - 1] == 'M' && chars[i - 2][j - 2] == 'A' && chars[i - 3][j - 3] == 'S' { 1 } else { 0 }
    } else { 0 };

    // check top right
    counter += if i + 3 < chars.len() && j + 3 < chars[i].len() {
        if chars[i + 1][j + 1] == 'M' && chars[i + 2][j + 2] == 'A' && chars[i + 3][j + 3] == 'S' { 1 } else { 0 }
    } else { 0 };

    // check bottom right
    counter += if i + 3 < chars.len() && j as i32 - 3 >= 0 {
        if chars[i + 1][j - 1] == 'M' && chars[i + 2][j - 2] == 'A' && chars[i + 3][j - 3] == 'S' { 1 } else { 0 }
    } else { 0 };

    // check top left
    counter += if i as i32 - 3 >= 0 && j + 3 < chars[i].len() {
        if chars[i - 1][j + 1] == 'M' && chars[i - 2][j + 2] == 'A' && chars[i - 3][j + 3] == 'S' { 1 } else { 0 }
    } else { 0 };

    counter
}

fn load_puzzle(filename: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let file = get_file(filename);
    let reader = io::BufReader::new(file);
    let characters: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();

    Ok(characters)
}
