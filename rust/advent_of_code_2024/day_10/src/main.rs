use std::{collections::{hash_set, HashMap, HashSet, VecDeque}, fs};

fn main() {
    println!("Part One: {:?}", part_one(parse_input("src/input")));
}

// make a que add all nodes to look at, at each position
// save valid moves to a Vec<Vec<Point>> so that we can 
// count the final Vec of 9s should probably not search
// the same path twice maybe allow for up and down move
// ment so all connected trails are found at the same time
// store wether we have visited a node so we can know 
//

fn parse_input(path: &str) -> Vec<Vec<usize>>{
    let mut output: Vec<Vec<usize>> = Vec::new();
    for line in fs::read_to_string(path).unwrap().lines(){
        let mut temp: Vec<usize> = Vec::new();
        for char in line.chars(){
            temp.push(char.to_string().parse::<usize>().unwrap());
        }
        output.push(temp);
    }
    output
}

#[derive(Debug, Hash, Eq, Clone, Copy)]
pub struct Point{
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Transform{
    pub x: i32,
    pub y: i32
}

impl Point {
   pub fn vector_to(&self, point:&Point) -> Transform{
    Transform{x:point.x as i32 - self.x as i32, y:point.y as i32 - self.y as i32}
   } 
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool{
        self.x == other.x && self.y == other.y
    }
}

impl Transform {
    pub fn transform_point(&self, point:&Point, grid: (usize, usize)) -> Option<Point>{
        let new_x: i32 = point.x as i32 + self.x;
        let new_y: i32 = point.y as i32 + self.y;
        if new_x >= 0 && new_y >= 0  && new_x <= grid.0 as i32 && new_y <= grid.1 as i32{
            return Some(Point{x:new_x as usize, y:new_y as usize});
        }else {
            return None;
        }
    }
}

const UP:Transform = Transform{y:1, x:0};
const DOWN:Transform = Transform{y:-1, x:0};
const RIGHT:Transform = Transform{y:0, x:1};
const LEFT:Transform = Transform{y:0, x:-1};


fn part_one(matrix: Vec<Vec<usize>>) -> usize{
    let valid_moves:Vec<Transform> = vec![UP, DOWN, RIGHT, LEFT];
    
    let mut grid_size: (usize, usize) = (0,0);
    let mut trails: Vec<HashMap<usize, HashSet<Point>>> = Vec::new();
    let mut que: VecDeque<Point> = VecDeque::new();
    
    //Init que state 
    for (row_index, row) in matrix.iter().enumerate(){
        for (col_index, col) in row.iter().enumerate(){
            if col == &0 {
                que.push_back(Point{x:col_index, y:row_index});
            }
            grid_size.0 = col_index;
        }
        grid_size.1 = row_index;
    }

    let mut current_trail: HashMap<usize, HashSet<Point>> = HashMap::new();
    while let Some(current_pos) = que.pop_back(){
        let current_point = matrix[current_pos.y][current_pos.x];
        if current_point == 0 && current_trail.len() > 0{
            // start new list
            trails.push(current_trail);
            current_trail = HashMap::new();
        }

        // add all valid moves to the que
        for transform in valid_moves.iter(){
            if let Some(new_point) = transform.transform_point(&current_pos, grid_size){
                let new_value = matrix[new_point.y][new_point.x];
                if new_value as i32 - current_point as i32 == 1{
                    que.push_back(new_point);
                }
            }
        }

        // add the current point
        if current_trail.contains_key(&current_point){
            current_trail.get_mut(&current_point).unwrap().insert(current_pos);
        } else {
            current_trail.insert(current_point, HashSet::from([current_pos]));
        }

    }
    let fake_hashset: HashSet<Point> = HashSet::new();
    
    trails.iter().map(|a| a.get(&9).unwrap_or(&fake_hashset).len()).sum()
}
