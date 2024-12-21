use std::fs;

fn main() {
    let part_one_input = parse_input("src/input_test");
    println!(
        "Part_one: {:?}",
        part_one(part_one_input.0, part_one_input.1)
    );
}

pub struct Movement {
    pub up: (i32, i32),
    pub right: (i32, i32),
    pub down: (i32, i32),
    pub left: (i32, i32),
}

impl Movement {
    fn new() -> Self {
        Movement {
            up: (0, 1),
            right: (1, 0),
            down: (0, -1),
            left: (-1, 0),
        }
    }
    fn rotate_90(rotate_from: (i32, i32), instance: Movement) -> (i32, i32){
        if rotate_from == instance.up{
            instance.right
        } else if rotate_from == instance.right {
            instance.down
        } else if rotate_from == instance.down {
            instance.left
        } else if rotate_from == instance.left{
            instance.up
        } else {
            panic!("no rotation");
        }
    }
}

fn parse_input(path: &str) -> (Vec<Vec<String>>, (usize, usize)) {
    let mut matrix: Vec<Vec<String>> = Vec::new();
    let guard = vec!["^", ">", "<", "v"];
    let mut guard_position: (usize, usize) = (0, 0);

    for line in fs::read_to_string(path).unwrap().lines() {
        matrix.push(
            line.chars()
                .map(|a| String::from(a))
                .collect::<Vec<String>>(),
        );
    }

    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if guard.contains(&&col[0..]) {
                guard_position = (row_index, col_index)
            }
        }
    }

    (matrix, guard_position)
}

fn determine_direciton(value: &str) -> (i32, i32) {
    let movement = Movement::new();
    match value {
        "^" => movement.up,
        ">" => movement.right,
        "<" => movement.left,
        "v" => movement.down,
        _ => panic!("movement not supported"),
    }
}

fn part_one(values: Vec<Vec<String>>, guard: (usize, usize)) -> i32 {
    let sum: i32 = 0;
    let path_finding = true;
    let starting_direction = determine_direciton(&values[guard.0][guard.1][0..]);
    while(path_finding){

    }

    sum
}
