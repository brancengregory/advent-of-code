fn read_input(p: &str) -> Vec<(i64, i64)> {
	std::fs::read_to_string(p).unwrap()
		.trim()
		.split(",")
		.map(|x| {
			let y = x.split("-").collect::<Vec<&str>>();
			(y[0].parse::<i64>().unwrap(), y[1].parse::<i64>().unwrap())
		})
		.collect()
}

fn check_invalid(n: i64) -> bool {
	let n_str = n.to_string();
	let n_digits = n_str.len();

	if n_digits % 2 != 0 { return false };

	n_str[0..(n_digits/2)] == n_str[(n_digits/2)..n_digits]
}

fn sum_invalids_in_range(a: i64, b: i64) -> i64 {
	let mut s: i64 = 0;
	for i in a..=b {
		if check_invalid(i) {
			s += i
		}
	}

	s
}

// Uses n to determine how many elements are in each chunk to compare, NOT the number of chunks
// So if s has 6 char we need to check 2 groups, 3 groups, 4 groups, etc. up to max number of
// groups equal to length of s
// That corresponds to chunk size 6/2 = 3, 6/3 = 2, 6/4 (invalid), 6/5 (invalid), 6/6 = 1 respectively
fn check_n_same(s: &str, n: usize) -> bool {
	let chars: Vec<char> = s.chars().collect();
	let len = chars.len();

	if n == 0 || len % n != 0 {
		return false;
	}

	let mut chunks = chars.chunks(n);

	let init = chunks.next().unwrap();

	for group in chunks {
		if group != init {
			return false;
		}
	}

	true
}

fn recheck_invalid(n: i64) -> bool {
	let n_str = n.to_string();
	let n_digits = n_str.len();

	for i in 1..=(n_digits/2) {
		if check_n_same(&n_str, i) {
			return true;
		}
	}

	false
}

fn resum_invalids_in_range(a: i64, b: i64) -> i64 {
	let mut s: i64 = 0;
	for i in a..=b {
		if recheck_invalid(i) {
			s += i
		}
	}

	s
}

fn main() {
	let input = read_input("../input");
  println!("{:?}", input);

  println!("{:?}", check_invalid(464));

	let ans: i64 = input.iter()
		.map(|x| {
			sum_invalids_in_range(x.0, x.1)
		})
		.sum();

  println!("{:?}", ans);

	let ans: i64 = input.iter()
		.map(|x| {
			resum_invalids_in_range(x.0, x.1)
		})
		.sum();

  println!("{:?}", ans);
}
