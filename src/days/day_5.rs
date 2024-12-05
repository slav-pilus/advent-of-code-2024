use crate::utils::files::get_file;
use std::io;
use std::io::BufRead;

pub fn run() {
    let input_file = "./input/day5.txt";
    let rules = load_rules(input_file).expect("Failed to read file");
    let updates = load_updates(input_file).expect("Failed to read file");

    process(&updates, &rules);
}

fn process(updates: &Vec<Vec<u32>>, rules: &Vec<Rule>) {
    let mut sum_of_middle_pages: u32 = 0;
    let mut sum_of_fixed_pages: u32 = 0;
    for update in updates {
        if is_valid_update(&rules, update) {
            let middle_page = update.get(update.len() / 2).unwrap();
            sum_of_middle_pages += middle_page;
        } else {
            sum_of_fixed_pages += fix_update(update, &rules).unwrap();
        }
    }

    println!("sum of middle pages of correct updates is: {}", sum_of_middle_pages);
    println!("sum of fixed pages is: {}", sum_of_fixed_pages);
}

fn is_valid_update(rules: &Vec<Rule>, update: &Vec<u32>) -> bool {
    let mut validity: Vec<bool> = Vec::new();
    for page in update {
        for r in rules.iter().filter(|x| x.left == page.clone()) {
            let valid = is_valid(update, page, r.right);
            validity.push(valid);
        }
    }
    validity.iter().all(|x| *x)
}

fn fix_update(update: &Vec<u32>, rules: &Vec<Rule>) -> Option<u32> {
    let mut new_update: Vec<u32> = Vec::new();
    new_update.push(update[0]);
    for i in 1..update.len() {
        let page = update[i];

        if let Some(_) = rules.iter().find(|x| x.left == page.clone() && x.right == new_update[i - 1]) {
            new_update.insert(new_update.iter().len() - 1, page)
        } else {
            new_update.push(page);
        }
    }

    if !is_valid_update(rules, &new_update) {
        fix_update(&new_update, rules)
    } else {
        new_update.get(new_update.len() / 2).copied()
    }
}

fn is_valid(update: &Vec<u32>, page: &u32, left: u32) -> bool {
    for i in 0..update.len() {
        if update[i] == page.clone() {
            return true;
        }
        if update[i] == left {
            return false;
        }
    }

    true
}

fn load_updates(filename: &str) -> Result<Vec<Vec<u32>>, std::io::Error> {
    let file = get_file(filename);
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let out = lines.iter()
        .filter(|l| l.contains(","))
        .map(|l| l.split(",")
            .map(|s| s.parse::<u32>().unwrap()).collect())
        .collect();

    Ok(out)
}

fn load_rules(filename: &str) -> Result<Vec<Rule>, std::io::Error> {
    let file = get_file(filename);
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut rules: Vec<Rule> = Vec::new();

    for line in lines {
        if line.contains("|") {
            let parts: Vec<u32> = line.split("|").map(|s| s.parse::<u32>().unwrap()).collect();
            rules.push(Rule::new(parts[0], parts[1]));
        }
    }

    Ok(rules)
}

struct Rule {
    left: u32,
    right: u32,
}

impl Rule {
    fn new(left: u32, right: u32) -> Rule {
        Rule { left, right }
    }
}
