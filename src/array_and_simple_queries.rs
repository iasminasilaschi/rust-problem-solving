use std::io::{self, BufRead};

pub(crate) fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read n and q
    let first_line: Vec<usize> = input
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let n = first_line[0];
    let q = first_line[1];

    // Read the array
    let mut arr: Vec<i64> = input
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // Process queries
    for _ in 0..q {
        let query: Vec<usize> = input
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let query_type = query[0];
        let i = query[1] - 1;
        let j = query[2];

        let segment: Vec<i64> = arr.drain(i..j).collect();

        if query_type == 1 {
            // Type 1: Add to the front
            arr.splice(0..0, segment);
        } else if query_type == 2 {
            // Type 2: Add to the back
            arr.extend(segment);
        }
    }

    // Calculate |A[0] - A[n-1]|
    let abs_diff = (arr[0] - arr[n - 1]).abs();
    println!("{}", abs_diff);

    // Print the resulting array
    println!(
        "{}",
        arr.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
