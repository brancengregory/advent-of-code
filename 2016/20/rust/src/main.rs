fn read_input(p: &str) -> Vec<(u64, u64)> {
	std::fs::read_to_string(p).unwrap()
		.lines()
		.map(|x| {
			let splits: Vec<&str> = x.split("-").collect();
			(
				splits[0].parse::<u64>().unwrap(),
				splits[1].parse::<u64>().unwrap()
			)
		})
		.collect()
}

fn main() {
	let input = read_input("../input");

	let mut init = input.clone();
	init.sort_by(|a, b| {
		a.0.cmp(&b.0)
	});

	let mut min_blocked = 0;
	let mut max_blocked = 0;
	let mut min_unblocked = 0;

	for r in &init {
		if (min_blocked..=max_blocked).contains(&r.0) {
			max_blocked = max_blocked.max(r.1);
		} else if r.0 == (max_blocked + 1) {
			max_blocked = max_blocked.max(r.1);
		} else {
			min_unblocked = max_blocked + 1;
			break;
		}
	}

	println!("{:?}", min_unblocked);

	let mut min_blocked = 0;
	let mut max_blocked = 0;
	let mut n_unblocked = 0;

	for r in &init {
		if (min_blocked..=max_blocked).contains(&r.0) {
			max_blocked = max_blocked.max(r.1);
		} else if r.0 == (max_blocked + 1) {
			max_blocked = max_blocked.max(r.1);
		} else {
			n_unblocked += r.0 - max_blocked - 1;
			max_blocked = r.1;
		}
	}

	println!("{:?}", n_unblocked);
}
