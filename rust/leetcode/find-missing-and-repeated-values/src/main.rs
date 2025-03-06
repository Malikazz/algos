fn main() {
    println!(
        "Output array {:?}",
        find_missing_and_repeated_values(vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]])
    );
}
pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut count_sort: Vec<i32> = vec![0; grid.len() * grid[0].len()];

    for sublist in grid.iter() {
        for value in sublist.iter() {
            count_sort[*value as usize - 1] += 1;
        }
    }

    let answer = count_sort
        .into_iter()
        .enumerate()
        .filter(|a| a.1 != 1)
        .collect::<Vec<(usize, i32)>>();
    if answer[0].1 == 0{
        return vec![(answer[1].0 +1) as i32, (answer[0].0 +1) as i32]
    } else {
        return vec![(answer[0].0 +1) as i32, (answer[1].0 +1) as i32]
    }
}
