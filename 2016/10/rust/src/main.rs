use std::collections::HashMap;

fn read_input(p: &str) -> Vec<String> {
    std::fs::read_to_string(p).unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let input = read_input("../input");

    let mut bots: HashMap<usize, Vec<i32>> = HashMap::new();
    let mut outputs: HashMap<usize, i32> = HashMap::new();
    let mut moves: HashMap<usize, ((&str, usize), (&str, usize))> = HashMap::new();

    input.iter()
        .for_each(|x| {
            let mut splits = x.split_whitespace().collect::<Vec<&str>>().into_iter();
            let command = splits.next().unwrap();
            match command {
                "value" => {
                    let n = splits.next().unwrap().parse::<i32>().unwrap(); 
                    let b = splits.last().unwrap().parse::<usize>().unwrap();
                    bots.entry(b).or_default().push(n);
                },
                "bot" => {
                    let from = splits.next().unwrap().parse::<usize>().unwrap();
                    let low_kind = splits.nth(3).unwrap();
                    let low_to = splits.next().unwrap().parse::<usize>().unwrap();
                    let high_kind = splits.nth(3).unwrap();
                    let high_to = splits.next().unwrap().parse::<usize>().unwrap();
                   
                    moves.insert(from, ((low_kind, low_to), (high_kind, high_to)));
                },
                _ => {},
            }
        });

    let (&init_bot, _) = bots.iter()
        .find(|(_, v)| {
            v.len() == 2
        })
        .unwrap();

    let mut next_bots = vec!(init_bot);

    while let Some(t) = &next_bots.pop() {
        let v = bots.get(t).unwrap();
        let v_min = *v.iter().min().unwrap();
        let v_max = *v.iter().max().unwrap();
        let m = moves.get(t).unwrap();
        
        match m.0.0 {
            "bot" => {
                bots.entry(m.0.1).or_default().push(v_min);
                if bots.get(&m.0.1).unwrap().len() == 2 {
                    next_bots.push(m.0.1);
                }
            },
            "output" => {
                outputs.insert(m.0.1, v_min);
            },
            _ => {},
        }

        match m.1.0 {
            "bot" => {
                bots.entry(m.1.1).or_default().push(v_max);
                if bots.get(&m.1.1).unwrap().len() == 2 {
                    next_bots.push(m.1.1);
                }
            },
            "output" => {
                outputs.insert(m.1.1, v_max);
            },
            _ => {},
        }
    }

    let target = bots.iter()
        .find(|(_, v)| v.contains(&61) && v.contains(&17))
        .map(|(&k, _)| k)
        .unwrap();

    println!("{:#?}", target);

    let output_product: i32 = [0, 1, 2].iter()
        .map(|k| outputs.get(k).unwrap())
        .product();

    println!("{:#?}", output_product);
}
