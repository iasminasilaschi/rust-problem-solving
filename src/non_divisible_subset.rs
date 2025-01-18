use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'non_divisible_subset' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY s
 */

fn non_divisible_subset(k: i32, s: &[i32]) -> i32 {
    let mut remainder_counts = vec![0; k as usize];

    // Count the frequency of each remainder
    for &num in s {
        let remainder = num % k;
        remainder_counts[remainder as usize] += 1;
    }

    let mut max_subset_size = 0;

    // Handle the 0 remainder group (at most one can be included)
    if remainder_counts[0] > 0 {
        max_subset_size += 1;
    }

    // Handle pairs of remainders that sum to k
    for i in 1..=(k / 2) {
        if i == k - i {
            // If k is even, include at most one element from the group with remainder k/2
            max_subset_size += 1;
        } else {
            max_subset_size += remainder_counts[i as usize].max(remainder_counts[(k - i) as usize]);
        }
    }

    max_subset_size
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

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let s: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = non_divisible_subset(k, &s);

    writeln!(&mut fptr, "{}", result).ok();
}
