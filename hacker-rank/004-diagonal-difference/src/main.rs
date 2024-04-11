use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    // Grab the matrix size
    let size = arr[0].len() as usize;
    println!("Matrix size: {}", size);
    println!("Matrix: {:?}", arr);

    // Compute diaganols
    let mut leftDiag = 0;
    let mut rightDiag = 0;
    for i in 0..size {
        leftDiag += arr[i][i];
        rightDiag += arr[size - i - 1][i];
        println!("Index {} Left {} Right {}", i, leftDiag, rightDiag);
    }

    // Compute difference
    (leftDiag - rightDiag).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
