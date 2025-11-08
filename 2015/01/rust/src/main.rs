use std::fs;

fn main() {
    let s: Vec<u8> = fs::read("../input").expect("Failed to read input file");

    let mut sum = 0;
    let mut basement = 0;

    for (index, e) in s.iter().enumerate() {
        let b = match *e {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        };

        sum += b;

        if basement == 0 && sum < 0 {
            basement = index + 1;
        }
    }

    println!("Part One: {}", sum);
    println!("Part Two: {}", basement);
}
