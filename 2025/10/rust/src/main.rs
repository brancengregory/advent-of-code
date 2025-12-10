#[derive(Debug)]
struct Machine {
    lights: Vec<u8>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>
}

fn read_input(p: &str) -> Vec<Machine> {
    std::fs::read_to_string(p).expect("Input file {} not found")
        .lines()
        .map(|x| {
            let mut splits = x.split_whitespace();

            let lights_str = splits.next().unwrap();

            let lights: Vec<u8> = lights_str
                .chars()
                .filter_map(|c| match c {
                    '.' => Some(0),
                    '#' => Some(1),
                    _ => None,
                })
                .collect();

            let joltages_str = splits.next_back().unwrap();

            let joltages: Vec<usize> = joltages_str
                .trim_matches(|c| c == '{' || c == '}')
                .split(",")
                .map(|s| {
                    s.parse::<usize>().unwrap()
                })
                .collect();
            
            let buttons = splits
                .map(|s| {
                    s.chars()
                        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
                        .collect()
                })
                .collect();
            
            Machine {
                lights,
                buttons,
                joltages,
            }
        })
        .collect()
}

fn main() {
    let input = read_input("../input");
    println!("{:#?}", input);
}
