fn build_next_row(row: &str) -> String {
    let row_bytes = row.as_bytes();
    let mut res = String::new();

    for i in 0..row.len() {
        let left_tile = if i < 1 { b'.' } else { row_bytes[i - 1] };
        let center_tile = row_bytes[i];
        let right_tile = if i == (row.len() - 1) { b'.' } else { row_bytes[i + 1] };
        
        let is_trap = match (left_tile, center_tile, right_tile) {
            (b'^', b'^', b'.') => true,
            (b'.', b'^', b'^') => true,
            (b'.', b'.', b'^') => true,
            (b'^', b'.', b'.') => true, 
            _ => false,
        };

        if is_trap {
            res.push_str("^")
        } else {
            res.push_str(".")
        }  
    }

    res
}

fn main() {
    let input = std::fs::read_to_string("../input").unwrap().trim().to_string();

    let mut res: Vec<String> = Vec::new();
    let mut last_row = input.clone();
    res.push(last_row.clone());

    for _ in 0..39 {
        let r = build_next_row(&last_row); 
        res.push(r.clone());
        last_row = r;
    }

    let n_safe = res.iter().fold(0, |acc, x| {
        let mut n = 0;
        for c in x.chars() {
            if c == '.' {
                n += 1
            }
        }
        acc + n
    });
    
    println!("{:?}", n_safe);

    let mut res: Vec<String> = Vec::new();
    let mut last_row = input;
    res.push(last_row.clone());

    for _ in 0..399999 {
        let r = build_next_row(&last_row); 
        res.push(r.clone());
        last_row = r;
    }

    let n_safe = res.iter().fold(0, |acc, x| {
        let mut n = 0;
        for c in x.chars() {
            if c == '.' {
                n += 1
            }
        }
        acc + n
    });
    
    println!("{:?}", n_safe);
}
