use std::collections::HashMap;

fn main() {
    println!("{:?}",fib(2));
    println!("{:?}",fib(0));
    println!("{:?}",fib(4));
}

pub fn fib(n: i32) -> i32{
    let mut memo:HashMap<i32,i32> = HashMap::new();
    
    if n <= 1 {
        return 1;
    }

    for num in 0..n +1{
        if num <= 1 {
            memo.insert(num, 1);
        }

        memo.insert(num, memo[&(num -1)] + memo[&(num-2)]);
    }

    memo[&n]
}
