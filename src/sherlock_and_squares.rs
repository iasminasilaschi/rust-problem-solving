use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'squares' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER a
 *  2. INTEGER b
 */

fn squares(a: i32, b: i32) -> i32 {
    let lower_bound = (a as f64).sqrt().ceil() as i32;
    let upper_bound = (b as f64).sqrt().floor() as i32;
    if lower_bound > upper_bound {
        0
    } else {
        upper_bound - lower_bound + 1
    }
}

pub(crate) fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..q {
        let first_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let a = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let b = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let result = squares(a, b);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
