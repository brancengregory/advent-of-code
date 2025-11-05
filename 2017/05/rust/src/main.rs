use std::fs::read_to_string;

fn read_input(p: &str) -> Vec<i32> {
	let content = read_to_string(p).unwrap();

	content
		.lines()
		.map(|x| {
			x.parse::<i32>().unwrap()
		})
		.collect()
}

fn main() {
  let input = read_input("../input");

	let mut instructions = input.clone();
	let mut current_location: isize = 0;
	let mut steps = 0;

	while current_location >= 0 && current_location < (instructions.len() as isize) {
		let index = current_location as usize;
		current_location = current_location + instructions[current_location as usize] as isize;
		instructions[index] += 1;

		steps += 1;
	}

	println!("{:?}", steps);

	instructions = input.clone();
	current_location = 0;
	steps = 0;

	while current_location >= 0 && current_location < (instructions.len() as isize) {
		let index = current_location as usize;
		let jump = instructions[index] as isize;
		current_location = current_location + jump;

		if jump >= 3 {
			instructions[index] -= 1;
		} else {
			instructions[index] += 1;
		}

		steps += 1;
	}

	println!("{:?}", steps);
}
