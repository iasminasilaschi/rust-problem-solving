use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'climbing_leaderboard' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY ranked
 *  2. INTEGER_ARRAY player
 */

fn climbing_leaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let mut unique_ranks: Vec<i32> = ranked.iter().copied().collect();
    unique_ranks.dedup(); // Keep only unique scores in descending order

    let mut player_ranks = Vec::new();
    let mut index = unique_ranks.len() as i32 - 1;

    for &score in player {
        while index >= 0 && score >= unique_ranks[index as usize] {
            index -= 1; // Move down the leaderboard
        }
        player_ranks.push(index + 2); // Calculate rank
    }

    player_ranks
}

pub(crate) fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ranked_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let ranked: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let _player_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let player: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = climbing_leaderboard(&ranked, &player);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
