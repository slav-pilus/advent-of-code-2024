use std::io;
use std::io::{BufRead};
use crate::utils::files::get_file;

pub fn run() {
    let input = read_lines("./input/day6.txt").expect("Failed to read lines");

    let visited_coordinates = part_1(input.clone());

    part_2(input.clone(), &visited_coordinates);
}

fn part_2(map: Vec<Vec<char>>, coordinates_to_block: &Vec<Coordinate>) {
    let mut counter = 0;
    for coordinate in coordinates_to_block {
        let mut reset_map = map.clone();
        let mut guard = find_guard(&reset_map);
        reset_map[coordinate.row][coordinate.col] = '#';
        let mut visited_locations = Vec::new();
        let mut looped = false;
        while can_move(&guard, &reset_map) && !looped {
            if can_move_forward(&guard, &mut reset_map) {
                move_guard(&mut guard, &mut reset_map);
                if !is_in_loop(&guard.coordinate, &visited_locations) {
                    looped = true;
                    counter += 1;
                } else {
                    visited_locations.push(guard.coordinate.clone());
                }
            } else {
                turn_guard(&mut guard);
            }
        }
    }

    println!("counter: {}", counter);
}

fn is_in_loop(current: &Coordinate, visited: &Vec<Coordinate>) -> bool {
    visited.iter().filter(|&x| x.col == current.col && x.row == current.row).count() < 4
}

fn part_1(mut map: Vec<Vec<char>>) -> Vec<Coordinate> {
    let mut guard = find_guard(&map);
    while can_move(&guard, &map) {
        if can_move_forward(&guard, &mut map) {
            move_guard(&mut guard, &mut map);
        } else {
            turn_guard(&mut guard);
        }
    }
    map[guard.coordinate.row][guard.coordinate.col] = 'X';

    let visited_coordinates = get_visited_coordinates(&mut map);
    println!("visited locations: {:?}", visited_coordinates.len());

    visited_coordinates
}

fn get_visited_coordinates(map: &mut Vec<Vec<char>>) -> Vec<Coordinate> {
    let mut coordinates: Vec<Coordinate> = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 'X' {
                coordinates.push(Coordinate { row, col });
            }
        }
    }
    coordinates
}

fn turn_guard(guard: &mut Location) {
    match guard.direction {
        Direction::Up => { guard.direction = Direction::Right }
        Direction::Down => { guard.direction = Direction::Left }
        Direction::Left => { guard.direction = Direction::Up }
        Direction::Right => { guard.direction = Direction::Down }
    }
}

fn can_move(location: &Location, map: &Vec<Vec<char>>) -> bool {
    match location.direction {
        Direction::Up => { location.coordinate.row as i32 - 1 >= 0 }
        Direction::Down => { location.coordinate.row + 1 < map.len() }
        Direction::Left => { location.coordinate.col as i32 - 1 >= 0 }
        Direction::Right => { location.coordinate.col + 1 < map[0].len() }
    }
}

fn can_move_forward(location: &Location, map: &Vec<Vec<char>>) -> bool {
    match location.direction {
        Direction::Up => { map[location.coordinate.row - 1][location.coordinate.col] != '#' }
        Direction::Down => { map[location.coordinate.row + 1][location.coordinate.col] != '#' }
        Direction::Left => { map[location.coordinate.row][location.coordinate.col - 1] != '#' }
        Direction::Right => { map[location.coordinate.row][location.coordinate.col + 1] != '#' }
    }
}

fn find_guard(map: &Vec<Vec<char>>) -> Location {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row][col] {
                '^' => return Location { coordinate: Coordinate { row, col }, direction: Direction::Up },
                '>' => return Location { coordinate: Coordinate { row, col }, direction: Direction::Right },
                '<' => return Location { coordinate: Coordinate { row, col }, direction: Direction::Left },
                'v' => return Location { coordinate: Coordinate { row, col }, direction: Direction::Down },
                _ => {}
            }
        }
    }

    panic!("Oh no, no guard found")
}

fn move_guard(location: &mut Location, map: &mut Vec<Vec<char>>) {
    let row = location.coordinate.row;
    let col = location.coordinate.col;
    match location.direction {
        Direction::Up => {
            location.coordinate.row -= 1;
            map[row][col] = 'X';
        }
        Direction::Down => {
            location.coordinate.row += 1;
            map[row][col] = 'X';
        }
        Direction::Left => {
            location.coordinate.col -= 1;
            map[row][col] = 'X';
        }
        Direction::Right => {
            location.coordinate.col += 1;
            map[row][col] = 'X';
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Location {
    coordinate: Coordinate,
    direction: Direction,
}

#[derive(Debug)]
struct Coordinate {
    row: usize,
    col: usize,
}
impl Coordinate {
    fn clone(&self) -> Coordinate {
        Coordinate { row: self.row, col: self.col }
    }
}

fn read_lines(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = get_file(filename);

    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let levels = lines.iter().map(|l| l.chars().collect()).collect();

    Ok(levels)
}
