use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    println!("Part_one {:?}", part_one(parse_input("src/input")));
}

// Seems like every point may need to track multi things 
// gonna move forward hoping that the diffculty in part two
#[derive(Debug, Hash, Eq)]
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

fn parse_input(path: &str) -> ((usize, usize), HashMap<String, Vec<Point>>){
    // need to know total grid size
    let lines = fs::read_to_string(path).unwrap();
    
    let grid_size:(usize, usize) = (lines.lines().map(|a| a.chars().count()).count() -1, lines.lines().count() -1 );
    let mut points: HashMap<String, Vec<Point>> = HashMap::new();
    
    for (y_index, line) in lines.lines().enumerate(){
        for (x_index, item) in line.chars().enumerate(){
            if item.to_string() == "."{
                continue;
            }
            if points.contains_key(&item.to_string()){
                points.get_mut(&item.to_string()).unwrap().push(Point{x:x_index, y:y_index});
            }else {
                points.insert(item.to_string(), vec![Point{x:x_index, y:y_index}]);
            } 
        }
    }
    (grid_size, points)
}


fn part_one(matrix: ((usize, usize), HashMap<String, Vec<Point>>)) -> usize{
    let grid = matrix.0;
    let matrix = matrix.1;
    let mut anti_nodes: HashSet<Point> = HashSet::new();
    for key in matrix.keys(){
        for location in matrix.get(key).unwrap().iter(){
            for compare_point in matrix.get(key).unwrap().iter(){
                if compare_point == location {
                    continue;
                }
                // get transform
                let transform:Transform = location.vector_to(compare_point);
                //apply transform  
                if let Some(first) = transform.transform_point(&compare_point, grid){
                    anti_nodes.insert(first);
                }
            }        
        }
    }
    // only count nodes on different points
    anti_nodes.iter().count()
}
