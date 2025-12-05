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

	let fresh: Vec<(u64, u64)> = fresh_str.iter()
		.map(|x| {
			(x[0].parse::<u64>().unwrap(), x[1].parse::<u64>().unwrap())
		})
		.collect();

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

	let mut fresh_ids: HashSet<u64> = HashSet::new();
	fresh.into_iter().for_each(|(start, end)| {
		for i in start..=end {
			fresh_ids.insert(i);
		}
	});

	println!("{:?}", fresh_ids);

	let ans2 = fresh_ids.len();
	println!("{}", ans2);
}
