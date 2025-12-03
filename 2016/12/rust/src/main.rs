use std::collections::HashMap;

fn read_input(p: &str) -> Vec<String> {
    std::fs::read_to_string(p).unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn compute<'a>(input: &'a Vec<String>, registers: &mut HashMap<&'a str, i32>) {
    let mut i: i32 = 0;
    
    loop {
        if i < 0 || i > (input.len() as i32 - 1) { break };

        let s = &input[i as usize];

        let mut splits = s.split_whitespace();
        let command = splits.next().unwrap();

        match command {
            "cpy" => {
                let from_str = splits.next().unwrap();
                let to = splits.next().unwrap();

                match from_str.parse::<i32>() {
                    Ok(from) => {
                        registers.insert(to, from);
                    },
                    Err(_) => {
                        let from = registers.get(from_str).unwrap();
                        registers.insert(to, *from);
                    },
                }
            },
            "inc" => {
                let v = splits.next().unwrap();
                registers.entry(v)
                    .and_modify(|x| *x += 1);
            },
            "dec" => {
                let v = splits.next().unwrap();
                registers.entry(v)
                    .and_modify(|x| *x -= 1);
            },
            "jnz" => {
                let v = splits.next().unwrap();
                let from: i32;

                if let Ok(x) = v.parse::<i32>() {
                    from = x;
                } else {
                    from = *registers.get(v).unwrap();
                }
                
                if from == 0 {
                    i += 1;
                    continue;
                };

                let n = splits.next().unwrap().parse::<i32>().unwrap();

                i += n;
                continue;
            },
            _ => panic!("Unexpected command"),
        }

        i += 1;
    }
}

fn main() {
    let input = read_input("../input");

    let mut registers: HashMap<&str, i32> = HashMap::new();
    registers.insert("a", 0);
    registers.insert("b", 0);
    registers.insert("c", 0);
    registers.insert("d", 0);

    compute(&input, &mut registers);

    let ans = registers.get("a").unwrap();
    println!("{:#?}", ans);

    registers.clear();
    registers.insert("a", 0);
    registers.insert("b", 0);
    registers.insert("c", 1);
    registers.insert("d", 0);

    compute(&input, &mut registers);

    let ans = registers.get("a").unwrap();
    println!("{:#?}", ans);
}
