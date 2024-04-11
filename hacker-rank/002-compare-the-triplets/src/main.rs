use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    // Track the score
    let mut aScore = 0;
    let mut bScore = 0;

    // Loop through challenge category score
    for i in 0..a.len() {
        if a[i] > b[i] {
            aScore += 1;
        } else if b[i] > a[i] {
            bScore += 1;
        }
        // Ignore addition for equals
    }

    vec![aScore, bScore]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compareTriplets(&a, &b);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
