use std::fs;

fn main() {
    let part_one_input = parse_input("src/input_test");
    println!(
        "Part_one: {:?}",
        part_one(part_one_input.0, part_one_input.1)
    );

    let part_two_input = parse_input("src/input_test");
    println!(
        "Part_two: {:?}",
        part_two(part_two_input.0, part_two_input.1)
    );
}

const UP: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (0, 1);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);

fn rotate_90(rotate_from: (i32, i32)) -> (i32, i32) {
    if rotate_from == UP {
        RIGHT
    } else if rotate_from == RIGHT {
        DOWN
    } else if rotate_from == DOWN {
        LEFT
    } else if rotate_from == LEFT {
        UP
    } else {
        panic!("no rotation");
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
    match value {
        "^" => UP,
        ">" => RIGHT,
        "<" => LEFT,
        "v" => DOWN,
        _ => panic!("movement not supported"),
    }
}

fn part_one(matrix: Vec<Vec<String>>, guard: (usize, usize)) -> i32 {
    let mut matrix = matrix.clone();
    let mut guard = guard.clone();
    let mut path_finding = true;
    let mut direction = determine_direciton(&matrix[guard.0][guard.1][0..]);
    let mut count: i32 = 0;
    while path_finding || count == 100000 {
        print_matrix(matrix.clone());
        count += 1;
        let next_move_index: (i32, i32) =
            (guard.0 as i32 + direction.0, guard.1 as i32 + direction.1);
        if next_move_index.0 < 0 || next_move_index.1 < 0 {
            path_finding = false;
            continue;
        }
        let fake_vec: Vec<String> = Vec::new();
        let next_move = matrix
            .get(next_move_index.0 as usize)
            .unwrap_or(&fake_vec)
            .get(next_move_index.1 as usize);
        if next_move.is_some() {
            let next_move_string =
                matrix[next_move_index.0 as usize][next_move_index.1 as usize].clone();
            match &next_move_string[0..] {
                "#" => direction = rotate_90(direction),
                "." | "X" => {
                    matrix[next_move_index.0 as usize][next_move_index.1 as usize] =
                        matrix[guard.0][guard.1].clone();
                    matrix[guard.0][guard.1] = String::from("X");
                    guard.0 = next_move_index.0 as usize;
                    guard.1 = next_move_index.1 as usize;
                }
                _ => panic!("guard could not move"),
            }
        } else {
            path_finding = false;
        }
    }
    // let the final state also be an X
    matrix[guard.0][guard.1] = String::from("X");
    print_matrix(matrix.clone());
    
    let mut sum = 0;
    for items in matrix.iter() {
        for item in items.iter() {
            if item == "X" {
                sum += 1;
            }
        }
    }
    sum
}

fn print_matrix(matrix: Vec<Vec<String>>) {
    println!("");
    for items in matrix.iter() {
        println!("{:?}", items);
    }
}

fn part_two(matrix: Vec<Vec<String>>, guard: (usize, usize)) -> i32 {
    0
}
