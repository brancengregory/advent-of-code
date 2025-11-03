use std::fs::read_to_string;
use itertools::Itertools;

fn read_input(p: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
	let content = read_to_string(p)?;

	Ok(
		content
			.chars()
			.filter_map(|c| c.to_digit(10))
			.collect()
	)
}

fn main() {
	let input = read_input("../input").unwrap();

	let first = input.iter().next();

	let total: u32 = input
		.iter()
		.chain(first)
		.tuple_windows::<(_, _)>()
		.filter_map(|(a, b)| {
			if a == b {
				Some(a)
			} else {
				None
			}
		})
		.sum();

	println!("{:?}", total);

	let delta = input.len() / 2;

	let shifted_input = input
		.iter()
		.cycle()
		.skip(delta);

	let new_total: u32 = input.iter()
		.zip(shifted_input)
		.filter_map(|(a, b)| {
			if a == b {
				Some(a)
			} else {
				None
			}
		})
		.sum();

	println!("{:?}", new_total);
}
