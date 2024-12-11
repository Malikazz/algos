use std::{collections::VecDeque, fs};
use log::debug;

enum Direction{
    Positive,
    Negative,
    UnSet
}

fn main() {
    println!("partone: {:?}", day_02_part_one(parse_input("src/input")));
    println!("parttwo: {:?}", day_02_part_two(parse_input("src/input")));
}

fn parse_input(path: &str) -> Vec<Vec<i32>>{
    let file_content = fs::read_to_string(path).unwrap();
    let mut report_levels: Vec<Vec<i32>> = Vec::new();

    // We could do all the logic here but could make part two hard to lets just incure
    // the cost of breaking it into two passes
    for line in file_content.lines() {
        let mut temp: Vec<i32> = Vec::new();
        for number in line.split(" ") {
            temp.push(number.parse::<i32>().unwrap());
        }
        report_levels.push(temp);
    }

    report_levels
}

fn calculate_diff_array(input: Vec<i32>) -> Vec<i32>{
    let mut output:Vec<i32> = Vec::new();
    for item in input.windows(2){
        output.push(item[1] - item[0]);
    }
    output
}

fn follows_rules(input: Vec<i32>) -> bool{
    let mut output: Vec<i32> = Vec::new();
    
    // Dose it follow the one direction rule
    let direction: 
    for item in input.iter(){
         
    }

    true
}

fn day_02_part_one(report_levels: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;
    'outer: for report in report_levels.iter() {
        let mut even_odd: i32 = -1;
        for numbers in report.windows(2) {
            if numbers[0] < numbers[1]
                && ((numbers[0] - numbers[1]).abs() <= 3 && (numbers[0] - numbers[1]).abs() > 0)
                && even_odd < 1
            {
                if even_odd == -1 {
                    even_odd = 0;
                }
                continue;
            } else if numbers[1] < numbers[0]
                && ((numbers[0] - numbers[1]).abs() <= 3 && (numbers[0] - numbers[1]).abs() > 0)
                && (even_odd == -1 || even_odd == 1)
            {
                if even_odd == -1 {
                    even_odd = 1;
                }
                continue;
            } else {
                continue 'outer;
            }
        }
        safe_reports += 1;
    }
    safe_reports
}

enum Direciton{
    Up,
    Down,
    UnSet
}

fn day_02_part_two(report_levels: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports: i32 = 0;
    let mut work_que:VecDeque<Vec<i32>> = VecDeque::new();
    
    for item in report_levels.into_iter(){
        work_que.push_back(item);
    }

    //evaluate if it passes all the rules
    while let Some(report) = work_que.pop_front(){
        let mut direction: Direciton = Direciton::UnSet; 
        for level in report.windows(3){
           // does 0 -> 1 follow the rules
           if (level[0] - level[1]).abs() > 0 && (level[0] - level[1]).abs() < 3 {
               if level[0] < level[1]{
                   if let Direciton::UnSet = direction{
                       direction = Direciton::Up;
                   }
                
               } 
           }
        }
    }
    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(day_02_part_one(parse_input("src/input_test")), 2);
    }

    #[test]
    fn test_two() {
        assert_eq!(day_02_part_two(parse_input("src/input_test")), 4);
    }
}
