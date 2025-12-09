use std::collections::{HashMap, BTreeSet};

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

fn fill_line(p1: &Point, p2: &Point, x_map: &HashMap<i64, usize>, y_map: &HashMap<i64, usize>, grid: &mut Vec<Vec<u8>>) {
	let x1 = x_map[&p1.x];
	let y1 = y_map[&p1.y];
	let x2 = x_map[&p2.x];
	let y2 = y_map[&p2.y];

	let start_x = x1.min(x2);
	let end_x = x1.max(x2);
	let start_y = y1.min(y2);
	let end_y = y1.max(y2);

	for y in start_y..=end_y {
		for x in start_x..=end_x {
			grid[y][x] = 1;
		}
	}
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

	let mut distinct_x = BTreeSet::new();
	let mut distinct_y = BTreeSet::new();

	for p in &input {
		distinct_x.insert(p.x);
		distinct_y.insert(p.y);
	}

	let sorted_x: Vec<i64> = distinct_x.into_iter().collect();
	let sorted_y: Vec<i64> = distinct_y.into_iter().collect();

	let x_map: HashMap<i64, usize> = sorted_x.iter().enumerate()
		.map(|(i, &v)| (v, i)).collect();
	let y_map: HashMap<i64, usize> = sorted_y.iter().enumerate()
		.map(|(i, &v)| (v, i)).collect();

	let width = sorted_x.len();
	let height = sorted_y.len();

	let mut grid: Vec<Vec<u8>> = vec![vec![0; width]; height];

	let num_points = input.len();
	for i in 0..num_points {
		let p1 = &input[i];
		let p2 = &input[(i + 1) % num_points];

		fill_line(p1, p2, &x_map, &y_map, &mut grid);
	}

}

