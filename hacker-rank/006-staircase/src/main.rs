use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    let maxStair: String = std::iter::repeat('#').take(n as usize).collect();
    for i in 1..=n as usize {
        let stair = &maxStair.as_str()[0..i];
        println!("{: >width$}", stair, width = n as usize)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}
