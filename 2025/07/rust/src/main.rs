fn read_input(p: &str) -> Vec<String> {
    std::fs::read_to_string(p).unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let input = read_input("../input");
    println!("{:#?}", input);
}
