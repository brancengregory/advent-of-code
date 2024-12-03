use regex::Regex;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("../input")?;

    let muls = Regex::new(r#"mul\(([0-9]{1, 3}),([0-9]{1, 3})\)"#).unwrap();
    let c: i32 = muls
        .captures_iter(&input)
        .map(|x| {
            let a = x[1].parse::<i32>().unwrap();
            let b = x[2].parse::<i32>().unwrap();

            a * b
        })
        .sum();

    println!("Part 1: {:?}", c);

    // Collect muls until reaching a don't()
    // Then search for a do()
    // Resume collecting muls until reaching a don't()
    // Repeat until the end of the input
    let dont = Regex::new(r#"don't\(\)"#).unwrap();
    let do_regex = Regex::new(r#"do\(\)"#).unwrap();

    let mut collecting = true;
    let mut total_sum = 0;
    let mut current_sum = 0;

    // Create an iterator for all matches (both `mul`, `don't()`, and `do()`)
    let combined_regex =
        Regex::new(r#"mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(\)|do\(\)"#).unwrap();

    let ans: i32 = combined_regex
        .find_iter(&input)
        .scan((true, 0), |state, mat| {
            let (collecting, current_sum) = state;
            let matched_str = mat.as_str();

            if dont.is_match(matched_str) {
                // Reached `don't()`
                *collecting = false;
                let result = Some(*current_sum);
                *current_sum = 0; // Reset the current sum
                result
            } else if do_regex.is_match(matched_str) {
                // Reached `do()`
                *collecting = true;
                Some(0) // No contribution to sum yet
            } else if let Some(caps) = muls.captures(matched_str) {
                // Reached `mul`
                if *collecting {
                    let a = caps[1].parse::<i32>().unwrap();
                    let b = caps[2].parse::<i32>().unwrap();
                    *current_sum += a * b;
                }
                Some(0) // No contribution to sum yet
            } else {
                Some(0) // Ignore unmatched input
            }
        })
        .sum();
    println!("Part 2: {:?}", ans);
    Ok(())
}
