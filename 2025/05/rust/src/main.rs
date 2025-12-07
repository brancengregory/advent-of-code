use std::collections::HashSet;

fn read_input(p: &str) -> Vec<String> {
	std::fs::read_to_string(p).unwrap()
		.lines()
		.map(String::from)
		.collect()
}

fn main() {
	let input = read_input("../input");

	let fresh_str: Vec<Vec<String>> = input.clone().into_iter()
		.take_while(|x| !x.is_empty())
		.map(|x| {
			x.split("-").map(String::from).collect()
		})
		.collect();

	let mut fresh: Vec<(u64, u64)> = fresh_str.iter()
		.map(|x| {
			(x[0].parse::<u64>().unwrap(), x[1].parse::<u64>().unwrap())
		})
		.collect();

	fresh.sort();

	let ids: Vec<u64> = input[fresh.len() + 1..input.len()].to_vec().iter().map(|x| x.parse::<u64>().unwrap()).collect();

	let ans: usize = ids.iter().fold(0, |acc, x| {
		if fresh.iter().any(|f| {
			(f.0..=f.1).contains(x)
		}) {
			acc + 1
		} else {
			acc
		}
	});

	println!("{}", ans);

	// TODO: too inefficient; need new strat
	//
	let mut fresh_ids: Vec<(u64, u64)> = Vec::new();
	println!("{:?}", fresh_ids);

	let mut fresh_iter = fresh.iter();
	let mut current = fresh.iter().next().unwrap();

	while let Some(t) = fresh_iter.next() {
		let covers_start = (current.0..=current.1).contains(&t.0);
		let covers_end = (current.0..=current.1).contains(&t.1);

		match (covers_start, covers_end) {
			(true, true) => { continue; },
			(true, false) => {
				current = (current.0, t.1)
			},
			_ => {
				fresh_ids.push(current);
				current = t;
			}
		}
	}

	let ans2 = fresh_ids.len();
	println!("{}", ans2);
}
