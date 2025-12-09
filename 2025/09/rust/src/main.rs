#[derive(Debug, Eq, PartialEq)]
struct Point {
	x: i64,
	y: i64,
}

fn read_input(p: &str) -> Vec<Point> {
	std::fs::read_to_string(p).unwrap()
		.lines()
		.map(|x| {
			let splits: Vec<&str> = x.split(",").collect();
			Point {
				x: splits[0].parse::<i64>().unwrap(),
				y: splits[1].parse::<i64>().unwrap(),
			}
		})
		.collect()
}

fn rectangular_area(a: &Point, b: &Point) -> i64 {
	let h = (b.y - a.y).abs() + 1;
	let w = (b.x - a.x).abs() + 1;

	h * w
}

fn main() {
	let input = read_input("../input");

	let mut max_area = 0;

	for i in 0..input.len() {
		for j in 0..input.len() {
			if i == j { continue };

			let a = &input[i];
			let b = &input[j];

			let area = rectangular_area(a, b);

			max_area = max_area.max(area);
		}
	}
  println!("{:?}", max_area);

}

