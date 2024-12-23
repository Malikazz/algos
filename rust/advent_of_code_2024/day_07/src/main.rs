use core::panic;
use itertools::{repeat_n, Itertools};
use std::{fs, u128};
// feels alot like the knapsack problem so maybe memoization
fn main() {
    println!("{:?}", part_one(parse_input("src/input")));
}

fn parse_input(path: &str) -> Vec<(u128, Vec<u128>)> {
    let mut output: Vec<(u128, Vec<u128>)> = Vec::new();
    for line in fs::read_to_string(path).unwrap().lines() {
        let parts = line.split(":").collect::<Vec<&str>>();
        let temp: (u128, Vec<u128>) = (
            parts[0].parse::<u128>().unwrap(),
            parts[1]
                .trim()
                .split(" ")
                .map(|a| a.parse::<u128>().unwrap())
                .collect::<Vec<u128>>(),
        );
        output.push(temp);
    }
    output
}

fn part_one(values: Vec<(u128, Vec<u128>)>) -> u128 {
    let mut sum:u128 = 0;
    let maths = vec!["+", "x"];

    'outer: for line in values.iter() {
        // is it to large or to small
        if line.1.iter().sum::<u128>() > line.0 || line.1.iter().product::<u128>() < line.0 {
            //skip
            continue;
        }

        // ok we have to process then
        for perm in repeat_n(maths.iter().cloned(), line.1.len() - 1).multi_cartesian_product() {
            let mut current_sum: u128 = line.1[0];
            for (index, operator) in perm.iter().enumerate() {
                match operator {
                    &"+" => current_sum += line.1[index + 1],
                    &"x" => {
                        current_sum *= line.1[index + 1]
                    }
                    _ => panic!("should never reach here"),
                }
                if current_sum == line.0 {
                    sum += current_sum;
                    continue 'outer;
                }
            }
        }
    }
    sum
}
