use std::cmp::min;
use std::{fs, usize};
fn main() {
    println!("part_one {:?}", part_one(load_string("src/input")));
    println!("part_two {:?}", part_two(load_string("src/input")));
}

fn load_string(path: &str) -> Vec<Vec<String>> {
    let mut matrix: Vec<Vec<String>> = Vec::new();

    for line in fs::read_to_string(path).unwrap().split("\r\n") {
        let mut temp: Vec<String> = Vec::new();
        for char in line.chars() {
            temp.push(String::from(char));
        }
        if temp.len() != 0 {
            matrix.push(temp);
        }
    }
    matrix
}

fn part_one(matrix: Vec<Vec<String>>) -> i32 {
    let mut sum = 0;

    // from any point atempt to look in all directions and match the string
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            let horizontal: String = matrix[row][col..min(col + 4, matrix[0].len())].join("");
            let vertical: String = matrix[row..min(row + 4, matrix.len())]
                .iter()
                .map(|a| a[col].clone())
                .collect::<Vec<String>>()
                .join("");
            let right_left: String = matrix[row..min(row + 4, matrix.len())]
                .iter()
                .enumerate()
                .map(|(index, a)| a.get(col + index).unwrap_or(&String::from("")).clone())
                .collect::<Vec<String>>()
                .join("");
            let left_right: String = matrix[row..min(row + 4, matrix.len())]
                .iter()
                .enumerate()
                .map(|(index, a)| {
                    let mut offset:i32 = col as i32 - index as i32;
                    if offset < 0 {
                        offset = i32::MAX;
                    }
                    a.get(offset as usize).unwrap_or(&String::from("")).clone()
                })
                .collect::<Vec<String>>()
                .join("");
           
            if horizontal == "XMAS" || horizontal == "SAMX"{
                sum += 1;
            }
            
            if vertical == "XMAS" || vertical == "SAMX"{
                sum += 1;
            }
            
            if right_left == "XMAS" || right_left == "SAMX"{
                sum += 1;
            }
            
            if left_right == "XMAS" || left_right == "SAMX"{
                sum += 1;
            }
        }
    }

    sum
}

fn part_two(matrix: Vec<Vec<String>>) -> i32{    
    let mut sum = 0;

    // from any point atempt to look in all directions and match the string
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            let right_left: String = matrix[row..min(row + 3, matrix.len())]
                .iter()
                .enumerate()
                .map(|(index, a)| a.get(col + index).unwrap_or(&String::from("")).clone())
                .collect::<Vec<String>>()
                .join("");
            let left_right: String = matrix[row..min(row + 3, matrix.len())]
                .iter()
                .enumerate()
                .map(|(index, a)| {
                    let mut offset:i32 = col as i32 - index as i32 + 2;
                    if offset < 0 {
                        offset = i32::MAX;
                    }
                    a.get(offset as usize).unwrap_or(&String::from("")).clone()
                })
                .collect::<Vec<String>>()
                .join("");
           
            if (right_left == "MAS" || right_left == "SAM") && (left_right == "MAS" || left_right == "SAM"){
                sum += 1;
            }
        }
    }
    sum
}
