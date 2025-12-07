fn read_input(p: &str) -> Vec<String> {
	std::fs::read_to_string(p).unwrap()
    .lines()
		.map(String::from)
		.collect()
}

fn main() {
    let input = read_input("../input");

    let operations_str = input.last().unwrap();
    let data = &input[0..input.len()-1];

    let operations: Vec<(usize, char)> = operations_str
        .char_indices()
        .filter(|(_i, c)| !c.is_whitespace())
        .collect();
    
    let mut ans: Vec<u64> = Vec::new();

    for i in 0..operations.len() { 
        let (current_idx, op_char) = operations[i];

        let end_idx = if i + 1 < operations.len() {
            Some(operations[i+1].0)
        } else {
            None
        };
        
        let mut col_val = match op_char { 
            '*' => 1,
            _ => 0,
        };

        for row in data {
            let slice_attempt = match end_idx {
                Some(end) => row.get(current_idx..end),
                None => row.get(current_idx..),
            };
            
            if let Some(slice) = slice_attempt { 
                if let Ok(num) = slice.trim().parse::<u64>() {
                    match op_char {
                        '+' => col_val += num,
                        '*' => col_val *= num,
                        _ => {},
                    }
                }
            }
        }

        ans.push(col_val);
    }

    println!("{:?}", ans.iter().sum::<u64>());

    let mut ans: Vec<u64> = Vec::new();

    for i in 0..operations.len() {
        let (start_idx, op_char) = operations[i];
        
        let end_idx = if i + 1 < operations.len() {
            operations[i+1].0
        } else {
            data.iter().map(|s| s.len()).max().unwrap_or(start_idx)
        };

        let mut block_result: u64 = match op_char {
            '*' => 1,
            _ => 0,
        };

        for col_idx in (start_idx..end_idx).rev() {
            let mut vertical_num_str = String::new();

            for row in data {
                if let Some(c) = row.chars().nth(col_idx) {
                    if c.is_digit(10) {
                        vertical_num_str.push(c);
                    }
                }
            }

            if !vertical_num_str.is_empty() {
                let num = vertical_num_str.parse::<u64>().unwrap();

                match op_char {
                    '+' => block_result += num,
                    '*' => block_result *= num,
                    _ => {},
                }
            }
        }
        ans.push(block_result);
    }

    println!("{:?}", ans.iter().sum::<u64>());
}
