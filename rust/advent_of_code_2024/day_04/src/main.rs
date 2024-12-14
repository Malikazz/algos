use std::fs;

fn main() {
    println!("part_one {:?}", part_one(load_string("src/input_test")))
}

pub struct GraphStrings{
    pub horizontal: Vec<String>,
    pub vertical: Vec<String>,
    pub positive_diagonal: Vec<String>,
    pub negative_diagonal: Vec<String>
}

impl GraphStrings {
   pub fn new() -> Self {
       return GraphStrings { horizontal: Vec::new(), vertical: Vec::new(), positive_diagonal: Vec::new(), negative_diagonal: Vec::new() } 
   } 
}


fn load_string(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap().lines().map(|c| String::from(c)).collect()
}

fn part_one(value: Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    // I think the move here to make things easy to understand
    // is to just use lots of memory and make everything a linear search
    
    let mut graphStrings: GraphStrings = GraphStrings::new();
    graphStrings.horizontal = value.clone();
    graphStrings.vertical = create_vertical(value.clone());
    graphStrings.positive_diagonal = create_diagonal_positive(value.clone());
    graphStrings.negative_diagonal = create_diagonal_negative(value.clone());

    sum += graphStrings.horizontal.iter().map(|c| c.matches("XMAS").count()).count() as i32;
    sum += graphStrings.horizontal.iter().map(|c| c.matches("SAMX").count()).count() as i32;
    
    sum += graphStrings.vertical.iter().map(|c| c.matches("XMAS").count()).count() as i32;
    sum += graphStrings.vertical.iter().map(|c| c.matches("SAMX").count()).count() as i32;
    
    sum += graphStrings.positive_diagonal.iter().map(|c| c.matches("XMAS").count()).count() as i32;
    sum += graphStrings.positive_diagonal.iter().map(|c| c.matches("SAMX").count()).count() as i32;
    
    sum += graphStrings.negative_diagonal.iter().map(|c| c.matches("XMAS").count()).count() as i32;
    sum += graphStrings.negative_diagonal.iter().map(|c| c.matches("SAMX").count()).count() as i32;

    sum
}

fn create_vertical(value: Vec<String>) -> Vec<String>{
    let mut vertical: Vec<String> = Vec::new();
    
    for _ in 0..value[0].len(){
        vertical.push(String::with_capacity(value.len()));
    }

    for item in value.iter(){
        for string_part in 0..item.len(){
            vertical[string_part] = vertical[string_part].to_owned() + &item[string_part..string_part];
        }
    }
    vertical
}

fn create_diagonal_positive(value: Vec<String>) -> Vec<String>{
    let mut temp: Vec<String> = Vec::new();

    for (index, item) in value.iter().enumerate(){
        temp.push(item[index..].to_owned());
        temp.push(item[0..index].to_owned());
    }

    temp
}

fn create_diagonal_negative(value: Vec<String>) -> Vec<String>{
    let mut temp: Vec<String> = Vec::new();
    for item in value.iter(){
        temp.push(item.chars().rev().collect());
    }
    
    return create_diagonal_positive(temp);
}
