

fn main() {
	let target_row: u64 = 3010;
	let target_col: u64 = 3019;

	let diag = target_row + target_col - 1;
	let start_index = (diag * (diag - 1)) / 2 + 1;
	let n = start_index + (target_col - 1);

	let mut current_code: u64 = 20151125;

	let mut codes: Vec<u64> = Vec::new();
	codes.push(current_code);

	for i in 1..n {
		let c = (current_code * 252533) % 33554393;
		codes.push(c);
		current_code = c;
	}

	println!("{:?}", &codes[(n-1) as usize]);
}

