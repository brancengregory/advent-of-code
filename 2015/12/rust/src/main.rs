use std::{fs::File, io::{Error, Read, Result}, ops::Add};
use serde_json::{Value};

fn read_input(p: &str) -> std::io::Result<Vec<String>> {
    let mut f = File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.lines().map(|x| x.to_string()).collect())
}

fn traverse_json(v: &Value) -> i64 {
    match v {
        Value::Object(map) => {
            map.values().map(|v| traverse_json(v)).sum()
        },
        Value::Array(arr) => {
            arr.iter().map(|x| traverse_json(x)).sum()
        },
        Value::Number(n) => {
            n.as_i64().unwrap_or(0)
        }
        _ => 0
    }
}

fn traverse_json_with_color(v: &Value) -> i64 {
    match v {
        Value::Object(map) => {
            if map.values().any(|x| x == "red") {
                0
            } else {
                map.values().map(|v| traverse_json_with_color(v)).sum()
            }
        },
        Value::Array(arr) => {
            arr.iter().map(|x| traverse_json_with_color(x)).sum()
        },
        Value::Number(n) => {
            n.as_i64().unwrap_or(0)
        }
        _ => 0
    }
}

fn main() -> std::io::Result<()>{
    let input = read_input("../input")?;

    let v: Value = serde_json::from_str(input[0].as_str())?;
    let ans1 = traverse_json(&v);
    let ans2 = traverse_json_with_color(&v);

    println!("Problem 1: {:?}", ans1);
    println!("Problem 2: {:?}", ans2);

    Ok(())
}
