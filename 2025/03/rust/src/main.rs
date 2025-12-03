fn read_input(p: &str) -> Vec<String> {
    std::fs::read_to_string(p).unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn find_bank_max(s: &str) -> i32 {
    let mut max_joltage = 0;
    
    let n: Vec<i32> = s.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    for i in 0..(n.len() - 1) {
        for j in (i + 1)..n.len() {
            let v = (10 * n[i]) + n[j];
            max_joltage = max_joltage.max(v);
        }
    }

    max_joltage
}

fn refind_bank_max(s: &str) -> i64 {
    let n: Vec<i64> = s.chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let mut res: Vec<i64> = Vec::new();
    let total_digits_needed = 12;
    
    let mut search_start = 0;
    
    while res.len() < total_digits_needed {
        let digits_still_needed = total_digits_needed - res.len();
        let search_end = n.len() - (digits_still_needed - 1);

        let mut max_val = -1;
        let mut max_idx = search_start;

        for i in search_start..search_end {
            let val = n[i];
            if val > max_val {
                max_val = val;
                max_idx = i;
                if val == 9 { break; }
            }
        }
        
        res.push(max_val);

        search_start = max_idx + 1;
    }

    res.iter()
        .fold(
            0,
            |acc, &x| {
                (acc * 10) + x
            }
        )
}

fn main() {
    let input = read_input("../input");

    let ans: i32 = input.iter()
        .map(|x| find_bank_max(&x))
        .sum();

    println!("{:#?}", ans);

    let ans: i64 = input.iter()
        .map(|x| refind_bank_max(&x))
        .sum();

    println!("{:#?}", ans);
}

