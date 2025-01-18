use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'array_manipulation' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. 2D_INTEGER_ARRAY queries
 */

fn array_manipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
    let mut arr = vec![0i64; (n + 1) as usize];

    // Apply range updates
    for query in queries {
        let a = query[0] as usize;
        let b = query[1] as usize;
        let k = query[2] as i64;

        arr[a] += k;
        if b + 1 <= n as usize {
            arr[b + 1] -= k;
        }
    }

    // Find the maximum value using prefix sums
    let mut max_value = 0;
    let mut current_sum = 0;
    for value in arr {
        current_sum += value;
        if current_sum > max_value {
            max_value = current_sum;
        }
    }

    max_value
}

pub(crate) fn main() {
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

    let mut queries: Vec<Vec<i32>> = Vec::with_capacity(m as usize);

    for i in 0..m as usize {
        queries.push(Vec::with_capacity(3_usize));

        queries[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = array_manipulation(n, &queries);

    writeln!(&mut fptr, "{}", result).ok();
}
