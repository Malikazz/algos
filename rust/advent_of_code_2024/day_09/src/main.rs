use std::{collections::VecDeque, fs};

fn main() {
    println!("Part one {:?}", part_one(parse_input("src/input_test")));
    println!("Part two {:?}", part_two(parse_input("src/input_test")));
}

#[derive(Debug)]
pub struct File {
    pub data: Vec<String>,
    pub index: usize,
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
        let mut temp: File = File {
            data: Vec::new(),
            index,
        };

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
            if back.data.len() > 0 {
                hard_drive.push_back(back);
            }
        } else {
            defraged_drive.push(next_empty);
            break;
        }

        if next_empty.get_avaiable_space() > 0 {
            hard_drive.push_front(next_empty);
        } else {
            defraged_drive.push(next_empty);
        }
    }
    //println!("{:?}", defraged_drive);
    calculate_checksum(defraged_drive)
}

fn part_two(hard_drive: VecDeque<File>) -> usize {
    let mut hard_drive = hard_drive;
    let mut defraged_drive: Vec<File> = Vec::new();
    let mut count = hard_drive.len();
    while let Some(mut current_back) = hard_drive.pop_back() {
        count -= 1;
        if hard_drive.len() == 0 {
            defraged_drive.push(current_back);
            break;
        }

        let size_requirement = current_back.data.len() - current_back.get_avaiable_space();
        // if some how the current block is all blank just add it to the output
        if size_requirement == 0 {
            defraged_drive.push(current_back);
            continue;
        }
        
        // need a base case
        let mut moved = false;
        for index in 0..hard_drive.len(){
            if hard_drive[index].get_avaiable_space() >= size_requirement{
                for replace_index in 0..hard_drive[index].data.len(){
                    if hard_drive[index].data[replace_index] == String::from("."){
                        if let Some(value) = current_back.data.pop(){
                            moved = true;
                            hard_drive[index].data[replace_index] = value;
                        }
                    }
                }
            }    
        }
        
        if moved == false && count == 0{
            // drain the vec
            while let Some(value) = hard_drive.pop_front(){
                defraged_drive.push(value);
            }
            defraged_drive.push(current_back);
        }
    }
    defraged_drive.sort_by(|a, b| a.index.cmp(&b.index));
    defraged_drive.iter().map(|a| println!("{:?}", a)).collect::<Vec<_>>();
    calculate_checksum(defraged_drive)
}
