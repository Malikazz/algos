use std::collections::HashSet;

fn main() {
    print!("{:?}", length_of_longest_substring(String::from("aaabcbbbbb")));
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut count: i32 = 0;
    let mut run: HashSet<char> = HashSet::new();

    for letter in s.chars(){
        if !run.insert(letter) {
            if count < run.len() as i32 {
                count = run.len() as i32;
                run.drain();
                run.insert(letter);
            }
        }
    }
    std::cmp::max(count, run.len() as i32)
}
