use regex::{bytes::Regex, RegexBuilder};
use std::fs::read_to_string;

fn main() {
    println!("Part one {:?}", part_one(load_input("src/input")));
    println!("Part two {:?}", part_two(load_input("src/input")));
}

fn load_input(path: &str) -> String {
    read_to_string(path).unwrap()
}

fn part_one(value: String) -> i32 {
    let regex: Regex = Regex::new(r"mul[(](\d+),(\d+)[)]").unwrap();
    let mut current_sum: i32 = 0;
    let instructions: Vec<(String, String)> = regex
        .captures_iter(value.as_bytes())
        .filter_map(|cap| {
            // Convert the captures into &str
            let key = std::str::from_utf8(&cap[1]).ok()?.to_string();
            let value = std::str::from_utf8(&cap[2]).ok()?.to_string();
            Some((key, value))
        })
        .collect();
    for int in instructions.iter() {
        current_sum += int.0.parse::<i32>().unwrap() * int.1.parse::<i32>().unwrap();
    }

    current_sum
}

// Apparently rust does not support variable length negative lockheads so lets just modify the
// string
fn pre_process(value: String) -> String {
    let regex = RegexBuilder::new(r"do\(\).*?don't\(\)").dot_matches_new_line(true).build().unwrap();
    let mut clean_str: String = String::with_capacity(value.len());

    for mat in regex.find_iter(&value) {
        clean_str += &value[mat.start()..mat.end()];
    }

    String::from(clean_str)
}

fn part_two(value: String) -> i32 {
    let value = pre_process(value);
    let regex = RegexBuilder::new(r"mul[(](\d+),(\d+)[)]").dot_matches_new_line(true).build().unwrap();
    let mut current_sum: i32 = 0;
    let instructions: Vec<(String, String)> = regex
        .captures_iter(&value)
        .filter_map(|cap| {
            // Convert the captures into &str
            let num1 = String::from(&cap[1]);
            let num2 = String::from(&cap[2]);
            Some((num1, num2))
        })
        .collect();
    for int in instructions.iter() {
        current_sum += int.0.parse::<i32>().unwrap() * int.1.parse::<i32>().unwrap();
    }

    current_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_process() {
        assert_eq!(
            pre_process(String::from(
                "this don't()sadfasdfasdfsdafdo()is a don't()asdfasdfdon't()do()test"
            )),
            "this is a test"
        );
    }

    #[test]
    fn test_pre_process_02() {
        assert_eq!(
            pre_process(String::from("don't()Do()asdfsdafsddo()this is a test")),
            "this is a test"
        );
    }
}
