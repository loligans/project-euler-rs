use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let mut input = arr.to_vec();
    input.sort();
    let small: i64 = input
        .iter()
        .take(4)
        .map(|&x| x as i64)
        .sum();
    let big: i64 = input
        .iter()
        .rev()
        .take(4)
        .map(|&x| x as i64)
        .sum();

    println!("{} {}", small, big);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
