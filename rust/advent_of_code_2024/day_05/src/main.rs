use std::{fs, usize};

fn main() {
    println!("part_one: {:?}", part_one(load_input("src/input")));
    println!("part_two: {:?}", part_two(load_input("src/input_test")));
}

fn load_input(path: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules: Vec<Vec<usize>> = Vec::new();
    let mut pages: Vec<Vec<usize>> = Vec::new();
    let input = fs::read_to_string(path).unwrap();

    for line in input.lines() {
        if line.contains("|") {
            let mut temp: Vec<usize> = Vec::new();
            for part in line.split("|") {
                temp.push(part.parse::<usize>().unwrap());
            }
            rules.push(temp);
        } else if line.contains(",") {
            let mut temp: Vec<usize> = Vec::new();
            for part in line.split(",") {
                temp.push(part.parse::<usize>().unwrap());
            }
            pages.push(temp);
        }
    }

    (rules, pages)
}

fn part_one(input: (Vec<Vec<usize>>, Vec<Vec<usize>>)) -> i32 {
    let mut sum: i32 = 0;
    let mut good: Vec<Vec<usize>> = Vec::new();
    // just brute force it ?
    'outer: for page in input.1.iter() {
        for rule in input.0.iter() {
            let before = page.iter().position(|x| x == &rule[0]).unwrap_or(0);
            let after = page
                .iter()
                .position(|x| x == &rule[1])
                .unwrap_or(usize::MAX);
            if before > after {
                continue 'outer;
            }
        }
        good.push(page.clone());
    }

    for item in good.iter() {
        sum += item[item.len() / 2] as i32;
    }

    sum
}

fn part_two(input: (Vec<Vec<usize>>, Vec<Vec<usize>>)) -> i32 {
    let mut sum: i32 = 0;
    let mut bad: Vec<Vec<usize>> = Vec::new();
    // just brute force it ?
    'outer: for page in input.1.iter() {
        for rule in input.0.iter() {
            let before = page.iter().position(|x| x == &rule[0]).unwrap_or(0);
            let after = page
                .iter()
                .position(|x| x == &rule[1])
                .unwrap_or(usize::MAX);
            if before > after {
                bad.push(page.clone());
                continue 'outer;
            }
        }
    }

    for item in bad.iter() {
        sum += item[item.len() / 2] as i32;
    }

    sum
}

fn my_sort(mut pages: Vec<Vec<usize>>, rules:Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    for item in pages.iter(){
        let sorting = true;
        for rule in rules.iter(){
            let before = item.iter().position(|x| x == &rule[0]).unwrap_or(0);
            let after = item
                .iter()
                .position(|x| x == &rule[1])
                .unwrap_or(usize::MAX);
            if before > after {
                item[]
            }
        }
    }

    pages
}
