use std::fs::read_to_string;
use itertools::Itertools;

fn read_input(p: &str) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
	let content = read_to_string(p)?;

	Ok(
		content
			.lines()
			.map(|x| x.parse::<i64>().unwrap())
			.collect()
	)
}

fn find_min_qe(input: &Vec<i64>, groups: i64) -> Option<i64> {
	let group_total: i64 = input.iter().sum::<i64>() / groups;
	let mut min_size = 1;
	let mut min_qe: Option<i64> = None;

	loop {
		let possible_groups: Vec<Vec<i64>> = input
			.iter()
			.copied()
			.combinations(min_size)
			.collect();

		let valid_groups: Vec<Vec<i64>> = possible_groups
			.into_iter()
			.filter(|group| group.iter().sum::<i64>() == group_total)
			.collect();

		if !valid_groups.is_empty() {
			println!("Found {} valid groups of size {}", valid_groups.len(), min_size);

			min_qe = valid_groups
				.iter()
				.map(|group| group.iter().product::<i64>())
				.min();

			break;
		}

		min_size += 1;

		if min_size >= input.len() / 3 {
			println!("No solution found");
			break;
		}
	}

	min_qe
}

fn main() {
	let mut input = read_input("../input").unwrap();
  input.sort();
	input.reverse();

	println!("{:?}", find_min_qe(&input, 3));
	println!("{:?}", find_min_qe(&input, 4));
}

