use std::{collections::VecDeque, fs};

fn main() {
    println!("Part one {:?}", part_one(parse_input("src/input_test")));
}

#[derive(Debug)]
pub struct File {
    pub data: Vec<String>,
}

impl File {
    pub fn get_avaiable_space(&self) -> usize {
        let mark = String::from(".");
        let count = &self.data.iter().filter(|a| a == &&mark).count();
        *count
    }
}

fn calculate_checksum(numbers: Vec<File>) -> usize {
    let mut sum = 0;
    let mut count = 0;
    for item in numbers.iter() {
        if item.get_avaiable_space() == 0 {
            for num in item.data.iter() {
                sum += num.parse::<usize>().unwrap() * count;
                count += 1;
            }
        }
    }
    sum
}

fn parse_input(path: &str) -> VecDeque<File> {
    let mut output: VecDeque<File> = VecDeque::new();
    for (index, char) in fs::read_to_string(path).unwrap().trim().chars().enumerate() {
        let mut temp: File = File { data: Vec::new() };

        if index % 2 == 0 || index == 0 {
            for _ in 0..(char.to_string().parse::<usize>().unwrap()) {
                if index != 0 {
                    temp.data.push((index / 2).to_string())
                } else {
                    temp.data.push(index.to_string())
                }
            }

            if temp.data.len() > 0 {
                output.push_back(temp);
            }
        } else {
            for _ in 0..(char.to_string().parse::<usize>().unwrap()) {
                temp.data.push(String::from("."));
            }

            if temp.data.len() > 0 {
                output.push_back(temp);
            }
        }
    }
    println!("{:?}", output);
    output
}

fn part_one(hard_drive: VecDeque<File>) -> usize {
    let mut hard_drive = hard_drive;
    let mut defraged_drive: Vec<File> = Vec::new();

    while let Some(mut next_empty) = hard_drive.pop_front() {
        // is this one not empty just add it to final
        let space = next_empty.get_avaiable_space();
        if space == 0 {
            defraged_drive.push(next_empty);
            continue;
        }

        // else lets add more stuff to it
        if let Some(mut back) = hard_drive.pop_back() {
            for index in 0..next_empty.data.len() {
                if next_empty.data[index] == String::from(".") {
                    if let Some(back_data) = back.data.pop() {
                        next_empty.data[index] = String::from(back_data);
                    }
                }
            }
        }
        
        if next_empty.get_avaiable_space() > 0 {
            hard_drive.push_front(next_empty);
        }
    }

    calculate_checksum(defraged_drive)
}
