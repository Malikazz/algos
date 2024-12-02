use std::fs;

fn main() {
    println!("partone: {:?}", day_02_part_one());
    println!("parttwo: {:?}", day_02_part_two());
}

fn day_02_part_one() -> i32 {
    let file_content = fs::read_to_string("src/input").unwrap();
    let mut report_levels: Vec<Vec<i32>> = Vec::new();
    let mut safe_reports = 0;

    // We could do all the logic here but could make part two hard to lets just incure
    // the cost of breaking it into two passes
    for line in file_content.lines() {
        let mut temp: Vec<i32> = Vec::new();
        for number in line.split(" ") {
            temp.push(number.parse::<i32>().unwrap());
        }
        report_levels.push(temp);
    }

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
                println!("List failed at index {:?}, report {:?}", index, report);
                continue 'outer;
            }
        }
        safe_reports += 1;
    }
    safe_reports
}

fn day_02_part_two() -> i32 {
    let file_content = fs::read_to_string("src/input").unwrap();
    let mut report_levels: Vec<Vec<i32>> = Vec::new();
    let mut safe_reports = 0;

    // We could do all the logic here but could make part two hard to lets just incure
    // the cost of breaking it into two passes
    for line in file_content.lines() {
        let mut temp: Vec<i32> = Vec::new();
        for number in line.split(" ") {
            temp.push(number.parse::<i32>().unwrap());
        }
        report_levels.push(temp);
    }

    'outer: for report in report_levels.iter_mut() {
        let mut even_odd: i32 = -1;
        let mut one_bad = 0;
        for index in (1..report.len()).rev() {
            let temp_left = report[index];
            let temp_right = report[index - 1];
            if temp_left < temp_right
                && ((temp_left - temp_right).abs() <= 3 && (temp_left - temp_right).abs() > 0)
                && (even_odd == 0 || even_odd == -1)
            {
                if even_odd == -1 {
                    even_odd = 0;
                }
                continue;
            } else if temp_right < temp_left
                && ((temp_left - temp_right).abs() <= 3 && (temp_left - temp_right).abs() > 0)
                && (even_odd == -1 || even_odd == 1)
            {
                if even_odd == -1 {
                    even_odd = 1;
                }
                continue;
            } else {
                if one_bad == 0 {
                    one_bad = 1;
                    report.remove(index - 1);
                    continue;
                } else {
                    continue 'outer;
                }
            }
        }
        safe_reports += 1;
    }
    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(day_02_part_one(), 2);
    }

    #[test]
    fn test_two() {
        assert_eq!(day_02_part_two(), 4);
    }
}
