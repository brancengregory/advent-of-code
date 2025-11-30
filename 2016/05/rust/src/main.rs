
fn read_input(p: &str) -> String {
    std::fs::read_to_string(p).unwrap().trim().to_string()
}

fn hash(p: &str, i: i32) -> md5::Digest {
    let s = format!("{}{}", p, i);
    md5::compute(s)
}

fn check_zeros(h: &md5::Digest) -> bool {
    h[0] == 0 && h[1] == 0 && h[2] < 0x10
}

fn main() {
    let input = read_input("../input");
    let mut index = 0;
    let mut password = String::new();

    while password.len() < 8 {
        let h = hash(&input, index);

        if check_zeros(&h) {
            let char_code = format!("{:x}", h[2] & 0x0F);
            println!("{}: {}", index, char_code);
            password.push_str(&char_code);
        }

        index += 1;
    }

    println!("Password: {}", password);

    index = 0;
    let mut found = 0;
    let mut password: [char; 8] = ['_'; 8];

    while found < 8 {
        let h = hash(&input, index);

        if check_zeros(&h) {
            let position = (h[2] & 0x0F) as usize;

            if position < 8 && password[position] == '_' {
                let val = h[3] >> 4;
                let char_char = std::char::from_digit(val as u32, 16).unwrap();
                password[position] = char_char;
                found += 1;
                println!("{}: {}: {}", position, char_char, password.iter().collect::<String>());
            }
        }

        index += 1;
    } 

    println!("Password: {:?}", password);
}
