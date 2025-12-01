fn read_input(p: &str) -> Vec<String> {
    std::fs::read_to_string(p).unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn has_abba(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    let ws = chars.windows(4);

    for w in ws {
        if w[0] != w[1] && w[0] == w[3] && w[1] == w[2] {
            return true;
        } 
    }

    false
}

fn get_abas(s: &str) -> Vec<String> {
    let mut abas: Vec<String> = Vec::new();
    let chars = s.chars().collect::<Vec<char>>();

    for w in chars.windows(3) {
        if w[0] != w[1] && w[0] == w[2] {
            let ss = w.iter().collect();
            abas.push(ss);
        }
    }

    abas
}

fn reverse_aba(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut rev_s = String::new();

    rev_s.push(chars[1]);
    rev_s.push(chars[0]);
    rev_s.push(chars[1]);

    rev_s
}

fn process_abbas_with_brackets(s: &str) -> bool {
    let mut buffer = String::new();
    let mut abba = false;
    let mut bracket_abba = false;
    
    for c in s.chars() {
        match c {
            '[' => {
                abba = abba || has_abba(&buffer);
                buffer.clear();
            },
            ']' => {
                bracket_abba = bracket_abba || has_abba(&buffer);
                buffer.clear();
            },
            _ => {
                buffer.push(c);
            }
        }
    }

    abba = abba || has_abba(&buffer);

    abba && !bracket_abba
}

fn process_abas_with_brackets(s: &str) -> bool {
    let mut buffer: String = String::new();
    let mut outer_abas: Vec<String> = Vec::new();
    let mut inner_abas: Vec<String> = Vec::new();

    for c in s.chars() {
        match c {
            '[' => {
                let new_outer_abas = get_abas(&buffer);
                outer_abas.extend(new_outer_abas);
                buffer.clear();
            },
            ']' => {
                let new_inner_abas = get_abas(&buffer);
                inner_abas.extend(new_inner_abas);
                buffer.clear();
            },
            _ => {
                buffer.push(c);
            }
        }
    }

    let new_outer_abas = get_abas(&buffer);
    outer_abas.extend(new_outer_abas);

    inner_abas.iter()
        .any(|x| {
            outer_abas.contains(&reverse_aba(x))
        })
}

fn main() {
    let input = read_input("../input");

    let abbas: Vec<&String> = input.iter().filter(|&x| process_abbas_with_brackets(x)).collect();
    println!("{:#?}", abbas.len());

    let abas: Vec<&String> = input.iter().filter(|&x| process_abas_with_brackets(x)).collect();
    println!("{:#?}", abas.len());
}
