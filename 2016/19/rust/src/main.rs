fn main() {
	let input = std::fs::read_to_string("../input").unwrap().trim().parse::<usize>().unwrap();

	let num_elves: usize = input;
	let mut presents = vec![1; num_elves];
	let mut next_elf: Vec<usize> = (0..num_elves).map(|i| (i + 1) % num_elves).collect();
	let mut current = 0;
	let mut remaining = num_elves;

	while remaining > 1 {
		let victim = next_elf[current];

		presents[current] += presents[victim];
		presents[victim] = 0;

		next_elf[current] = next_elf[victim]; // This covers skipping logic

		current = next_elf[current];

		remaining -= 1;
	}

	println!("{:?}", current + 1);

	let num_elves: usize = input;
	let mut prev: Vec<usize> = (0..num_elves).map(|i| (i+num_elves-1) % num_elves).collect();
	let mut next: Vec<usize> = (0..num_elves).map(|i| (i+1) % num_elves).collect();
	let mut presents = vec![1; num_elves];
	let mut current = 0;
	let mut victim = num_elves / 2;

	for remaining in (1..num_elves).rev() {
		presents[current] += presents[victim];
		presents[victim] = 0;

		let v_left = prev[victim];
		let v_right = next[victim];

		next[v_left] = v_right;
		prev[v_right] = v_left;

		if remaining % 2 == 0 {
			victim = next[v_right];
		} else {
			victim = v_right;
		}

		current = next[current];
	}

	println!("{:?}", current + 1);
}
