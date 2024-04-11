#[allow(dead_code)]
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'getTotalX' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    a / gcd(a, b) * b
}

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    // Calculate the LCM of a and take the largest LCM
    let lcm_arr = a.iter().fold(1, |acc, &x| lcm(acc, x));
    // Calculate the GCD of b and take the largest GCD
    let gcd_arr = b.iter().fold(b[0], |acc, &x| gcd(acc, x));
    let mut count = 0;

    println!("LCM Arr: {:?}", lcm_arr);
    println!("GCD Arr: {:?}", gcd_arr);

    let mut multiple = lcm_arr;
    while multiple <= gcd_arr {
        if gcd_arr % multiple == 0 {
            count += 1;
        }

        multiple += lcm_arr
    }

    println!("Total multiples between a and b: {}", count);
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}
