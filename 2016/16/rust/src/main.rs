fn flip_bits(s: &str) -> String {
    s.chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect()
}

fn reverse_str(s: &str) -> String {
    s.chars().rev().collect()
}

fn dragon_curve(a: &str) -> String {
    let mut b = reverse_str(a);
    b = flip_bits(&b);
    format!("{}0{}", a, b)
}

fn checksum(s: &str) -> String {
    let s_bytes = s.as_bytes();
    
    if s_bytes.len() % 2 != 0 {
        panic!("Can't compute checksum on odd number of characters")
    }
    
    let mut res = String::new();
    
    for b in s_bytes.chunks(2) {
        if b[0] == b[1] {
            res.push_str("1")
        } else {
            res.push_str("0")
        }
    }

    if res.len() % 2 == 0 {
        res = checksum(&res)
    }

    res
}

fn fill_to_n(input: &str, n: usize) -> String {
    let mut new = input.to_string();

    while new.len() < n {
        new = dragon_curve(&new);
    }

    new.truncate(n);

    new
}

fn main() {
    let input = std::fs::read_to_string("../input").unwrap().trim().to_string();

    let mut res = fill_to_n(&input, 272); 
    res = checksum(&res);
    println!("{:?}", res);

    res = fill_to_n(&input, 35651584); 
    res = checksum(&res);
    println!("{:?}", res);
}

