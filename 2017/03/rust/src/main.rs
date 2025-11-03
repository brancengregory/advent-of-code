
fn main() {
	let val = 312051;
	let mut ring: i32 = 0;
	let mut odd: i32 = 1;
	let mut number: i32 = odd.pow(2);

	while number < val {
		ring += 1;
		odd += 2;
		number = odd.pow(2);
	}

	println!("ring: {}, odd: {}, number: {}", ring, odd, number);

	let mut coords: (i32, i32) = (ring, ring);
	let side_length = ring * 2;
	println!("coords: {:?}", coords);
	println!("distance: {:?}", coords.0.abs() + coords.1.abs());

	let diff = number - val;
	println!("diff: {:?}", diff);

	if !(diff < side_length) { panic!("diff ({}) not less than side length ({})", diff, side_length) };

	coords.1 -= diff;
	println!("coords: {:?}", coords);
	println!("distance: {:?}", coords.0.abs() + coords.1.abs());
}

