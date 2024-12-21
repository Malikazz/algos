use itertools::Itertools;
use std::fs;

fn main() {
    println!("partone: {:?}", day_02_part_one(parse_input("src/input")));
    println!("parttwo: {:?}", day_02_part_two());
}

fn parse_input(path: &str) -> Vec<Vec<i32>> {
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

fn day_02_part_two(nums: Vec<i32>) -> usize {
    (0..nums.len())
        .any(|i| {
            nums[0..i]
                .iter()
                .chain(&nums[i + 1..])
                .tuple_windows()
                .try_fold(0, |ord, (a, b)| {
                    if ord >= 0 && (1..=3).contains(&(b - a)) {
                        Ok(1)
                    } else if ord <= 0 && (1..=3).contains(&(a - b)) {
                        Ok(-1)
                    } else {
                        Err(())
                    }
                })
                .is_ok()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(day_02_part_one(parse_input("src/input_test")), 2);
    }
}
