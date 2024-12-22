use core::panic;
use std::fs;
use itertools::{iproduct, repeat_n, Itertools};
// feels alot like the knapsack problem so maybe memoization
fn main() {
    println!("{:?}", part_one(parse_input("src/input_test")));
}

fn parse_input(path: &str) -> Vec<(usize, Vec<usize>)> {
    let mut output: Vec<(usize, Vec<usize>)> = Vec::new();
    for line in fs::read_to_string(path).unwrap().lines() {
        let parts = line.split(":").collect::<Vec<&str>>();
        let temp: (usize, Vec<usize>) = (
            parts[0].parse::<usize>().unwrap(),
            parts[1]
                .trim()
                .split(" ")
                .map(|a| a.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        );
        output.push(temp);
    }
    output
}

fn multiply_all_numbers(numbers: &Vec<usize>) -> usize {
    let mut sum = 1;
    for number in numbers.iter() {
        sum *= number;
    }
    sum
}

fn part_one(values: Vec<(usize, Vec<usize>)>) -> usize {
    let mut sum = 0;
    let maths = vec!["+", "x"];

   'outer: for line in values.iter() {
        // is it to large
        if line.0 > multiply_all_numbers(&line.1) {
            continue;
        }
        // is it too small
        if line.0
            < line
                .1
                .windows(2)
                .map(|a| a[0] + a[1])
                .collect::<Vec<usize>>()
                .iter()
                .sum()
        {
            continue;
        }

        // ok we have to process then
        for perm in repeat_n(maths.iter().cloned(), line.1.len() - 1).multi_cartesian_product(){
        let mut current_sum:usize = line.1[0];
        println!("{:?}", line.1);
            for (index,operator) in perm.iter().enumerate(){
                println!("{:?}", operator);
            //println!("Testing if current_sum {:?} = target {:?} then {:?} current sum by {:?}", current_sum, line.0, operator, line.1[index]);
                match operator{
                    &"+" => current_sum += line.1[index + 1],
                    &"x" => { if current_sum == 0{ current_sum = 1} current_sum *= line.1[index + 1 ]},
                    _ => panic!("should never reach here")
                }
                if current_sum == line.0 {
                    println!("Current sum: {:?} adding {:?}", sum, current_sum);
                    sum += current_sum;
                    continue 'outer;
                }
            }
        }
    }

    sum
}
