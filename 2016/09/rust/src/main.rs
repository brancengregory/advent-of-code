fn read_input(p: &str) -> String {
    std::fs::read_to_string(p).unwrap().trim().to_string()
}

fn main() {
    let input = read_input("../input");

    let mut d_str = String::new();
    let mut input_chars = input.chars();
    let mut in_command = false;
    
    let mut buffer = String::new();
    
    while let Some(c) = input_chars.next() {
        if in_command {
            match c {
                ')' => {
                    let mut splits = buffer.split("x");
                    let n_chars = splits.next().unwrap().parse::<usize>().unwrap();
                    let n_repeats = splits.next().unwrap().parse::<usize>().unwrap();

                    // take n_chars chars from input_chars
                    // repeat it n_repeats times
                    let b = input_chars.by_ref()
                        .take(n_chars)
                        .collect::<String>()
                        .repeat(n_repeats);

                    // add repeated content to decrypted string
                    d_str.push_str(&b);

                    buffer.clear();
                    in_command = false;
                },
                _ => buffer.push(c),
            }
        } else {
            match c {
                '(' => {
                    in_command = true;
                },
                _ => d_str.push(c),
            }
        }
    }

    println!("{:#?}", d_str.len());

    let d_str_len = count_decrypted_chars(&input);

    println!("{:#?}", d_str_len);
}

fn count_decrypted_chars(s: &str) -> usize {
    let mut in_command = false;
    let mut buffer = String::new();
    let mut s_chars = s.chars();
    let mut d_str_len = 0;

    while let Some(c) = s_chars.next() {
        if in_command {
            match c {
                ')' => {
                    let mut splits = buffer.split("x");
                    let n_chars = splits.next().unwrap().parse::<usize>().unwrap();
                    let n_repeats = splits.next().unwrap().parse::<usize>().unwrap();

                    let ss: String = s_chars.by_ref()
                        .take(n_chars)
                        .collect();

                    let expanded_len = count_decrypted_chars(&ss);
                    
                    d_str_len += expanded_len * n_repeats;
                    
                    buffer.clear();
                    in_command = false;
                },
                _ => buffer.push(c),
            }
        } else {
            match c {
                '(' => {
                    in_command = true;
                },
                _ => d_str_len += 1,
            }
        }
    }
    
    d_str_len
}
