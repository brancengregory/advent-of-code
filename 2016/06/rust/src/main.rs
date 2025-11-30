use std::collections::HashMap;

fn read_input(p: &str) -> Vec<String> {
    std::fs::read_to_string(p).unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn count_char_positions(hm: &mut HashMap<(usize, char), u32>, s: &str) {
    for (i, c) in s.chars().enumerate() {
        hm.entry((i, c))
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }
}

fn main() {
    let input = read_input("../input");

    let mut hm: HashMap<(usize, char), u32> = HashMap::new();

    input.iter()
        .for_each(|x| count_char_positions(&mut hm, x));

    let mut message = String::new();

    for i in 0..8 {
        let v = hm.iter()
            .filter(|(&x, _)| x.0 == i)
            .max_by(|&x, &y| {
                x.1.cmp(y.1)
            })
            .unwrap();

        message.push(v.0.1)
    }
    
    println!("{:?}", message);

    message = String::new();
    for i in 0..8 {
        let v = hm.iter()
            .filter(|(&x, _)| x.0 == i)
            .min_by(|&x, &y| {
                x.1.cmp(y.1)
            })
            .unwrap();

        message.push(v.0.1)
    }

    println!("{:?}", message)
}
