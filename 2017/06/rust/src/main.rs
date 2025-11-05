use std::fs::read_to_string;
use std::collections::HashSet;

fn read_input(p: &str) -> Vec<u32> {
	let content = read_to_string(p).unwrap();

	content
		.split_whitespace()
		.map(|c| c.parse::<u32>().unwrap())
		.collect()
}

fn main() {
  let input = read_input("../input");

	let mut banks = input.clone();
	let mut cycles = 0;
	let mut configs: HashSet<Vec<u32>> = HashSet::new();

	while configs.insert(banks.clone()) {
		cycles += 1;

		let (max_bank, max_val) = banks
			.iter()
			.enumerate()
			.max_by_key(|&(i, &x)| (x, std::cmp::Reverse(i)))
			.map(|(i, &x)| (i, x))
			.unwrap();

		banks[max_bank] = 0;

		for i in 1..=max_val {
			let j = (max_bank + i as usize) % banks.len();
			banks[j] += 1;
		}
	}

	println!("{:?}", cycles);

	let mut cycles = 0;
	let dup_config = banks.clone();

	loop {
		cycles += 1;

		let (max_bank, max_val) = banks
			.iter()
			.enumerate()
			.max_by_key(|&(i, &x)| (x, std::cmp::Reverse(i)))
			.map(|(i, &x)| (i, x))
			.unwrap();

		banks[max_bank] = 0;

		for i in 1..=max_val {
			let j = (max_bank + i as usize) % banks.len();
			banks[j] += 1;
		}

		if banks == dup_config {
			break;
		}
	}

	println!("{:?}", cycles);
}
