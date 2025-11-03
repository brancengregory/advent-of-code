use std::fs::read_to_string;
use itertools::Itertools;

fn read_input(p: &str) -> Result<Vec<Vec<u32>>, Box<dyn std::error::Error>> {
	let content = read_to_string(p)?;

	Ok(
		content
			.lines()
			.map(|x| {
				x.to_string().split_whitespace().map(|c| c.parse::<u32>().unwrap()).collect()
			})
			.collect()
	)
}

fn divide_divisibles(v: &Vec<u32>) -> u32 {
	let mut t = v.clone();
	t.sort_unstable_by(|a, b| b.cmp(a));
	t
		.iter()
		.tuple_combinations::<(_, _)>()
		.filter_map(|(a, b)| {
			if a % b == 0 {
				Some(a / b)
			} else {
				None
			}
		})
		.next()
		.unwrap()
}

fn main() {
	let input = read_input("../input").unwrap();

	let res: u32 = input
		.iter()
		.map(|r| {
			r.iter().max().unwrap() - r.iter().min().unwrap()
		})
		.sum();

  println!("{:?}", res);

	let new_res: u32 = input
		.iter()
		.map(divide_divisibles)
		.sum();

  println!("{:?}", new_res);
}
