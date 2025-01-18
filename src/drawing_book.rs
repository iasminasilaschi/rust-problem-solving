use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'page_count' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER p
 */

fn page_count(n: i32, p: i32) -> i32 {
    // Calculate pages to turn from the front
    let from_front = p / 2;

    // Calculate pages to turn from the back
    let from_back = (n / 2) - (p / 2);

    // Return the minimum of the two
    from_front.min(from_back)
}

pub(crate) fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let p = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let result = page_count(n, p);

    writeln!(&mut fptr, "{}", result).ok();
}
