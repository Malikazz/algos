use std::fs;

fn main() {
    println!("Hello, world!");
}



fn day_01_part_one() ->  i32 {
    let file_content = fs::read_to_string("src/input").unwrap();
    let mut vec_left:Vec<i32> = Vec::new();
    let mut vec_right:Vec<i32> = Vec::new();
    for line in file_content.lines(){
        let mut sides = line.split_whitespace();
        vec_left.push(sides.next().unwrap().parse::<i32>().unwrap());
        vec_right.push(sides.next().unwrap().parse::<i32>().unwrap());
    }
    
    vec_left.sort();
    vec_right.sort();
    
    vec_left.iter().zip(vec_right.iter()).map(|(a,b)| (a-b).abs()).sum()
}

fn day_01_part_two() -> i32 {
     let file_content = fs::read_to_string("src/input").unwrap();
    let mut vec_left:Vec<i32> = Vec::new();
    let mut vec_right:Vec<i32> = Vec::new();
    for line in file_content.lines(){
        let mut sides = line.split_whitespace();
        vec_left.push(sides.next().unwrap().parse::<i32>().unwrap());
        vec_right.push(sides.next().unwrap().parse::<i32>().unwrap());
    }
    
    vec_left.sort();
    vec_right.sort();
    
    let mut total:i32 = 0;

    for (index, left) in vec_left.iter().enumerate(){
        for right in vec_right[index..].iter(){
            if right > left{
                break;
            }
            if left == right{
                total += left;
            }
        }    
    }

    total

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(day_01_part_one(), 0);
    }
    
    #[test]
    fn test_two() {
        assert_eq!(day_01_part_two(), 0);
    }
}


