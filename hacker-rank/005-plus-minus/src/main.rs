use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    // Track occurrences of pos, neg, zero
    let mut pos = 0;
    let mut neg = 0;
    let mut zero = 0;
    for &value in arr.iter() {
       if value > 0 {
           pos += 1;
       } else if value < 0 {
           neg += 1;
       } else {
           zero += 1;
       }
    };

    let size = arr.len() as f64;
    println!("{:.6}", pos as f64 / size);
    println!("{:.6}", neg as f64 / size);
    println!("{:.6}", zero as f64 / size);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
