use std::{fs::File, io::Read};
use std::fmt::Debug;
use regex::Regex;

use faer::Mat;

#[derive(Clone)]
struct Grid {
    mat: Mat<f64>
}

impl Grid {
    fn turn_on(&mut self, start: Point, end: Point) {
        let m = &mut self.mat;

        for i in start.x..=end.x {
            for j in start.y..=end.y {
                m[(i as usize, j as usize)] = f64::from(1);
            }
        }
    }

    fn nordic_turn_on(&mut self, start: Point, end: Point) {
        let m = &mut self.mat;

        for i in start.x..=end.x {
            for j in start.y..=end.y {
                let c = m[(i as usize, j as usize)];
                m[(i as usize, j as usize)] = c + f64::from(1);
            }
        }
    }

    fn toggle(&mut self, start: Point, end: Point) {
        let m = &mut self.mat;
        
        for i in start.x..=end.x {
            for j in start.y..=end.y {
                m[(i as usize, j as usize)] = match m[(i as usize, j as usize)] {
                    0.0 => f64::from(1),
                    1.0 => f64::from(0),
                    _ => panic!()
                };
            }
        }
    }

    fn nordic_toggle(&mut self, start: Point, end: Point) {
        let m = &mut self.mat;

        for i in start.x..=end.x {
            for j in start.y..=end.y {
                let c = m[(i as usize, j as usize)];
                m[(i as usize, j as usize)] = c + f64::from(2);
            }
        }
    }

    fn turn_off(&mut self, start: Point, end: Point) {
        let m = &mut self.mat;

        for i in start.x..=end.x {
            for j in start.y..=end.y {
                m[(i as usize, j as usize)] = f64::from(0);
            }
        }
    }

    fn nordic_turn_off(&mut self, start: Point, end: Point) {
        let m = &mut self.mat;

        for i in start.x..=end.x {
            for j in start.y..=end.y {
                let c = m[(i as usize, j as usize)];
                if c - f64::from(1) < f64::from(0) {
                    m[(i as usize, j as usize)] = f64::from(0);
                } else {
                    m[(i as usize, j as usize)] = c - f64::from(1);
                }
            }
        }
    }

    fn follow_instruction(&mut self, i: Instruction) {
        match i.a {
            Action::TurnOn => self.turn_on(i.s, i.e),
            Action::Toggle => self.toggle(i.s, i.e),
            Action::TurnOff => self.turn_off(i.s, i.e)
        };
    }

    fn follow_nordic_instruction(&mut self, i: Instruction) {
        match i.a {
            Action::TurnOn => self.nordic_turn_on(i.s, i.e),
            Action::Toggle => self.nordic_toggle(i.s, i.e),
            Action::TurnOff => self.nordic_turn_off(i.s, i.e)
        };
    }
}

#[derive(Debug, Clone)]
enum Action {
    TurnOn,
    Toggle,
    TurnOff
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "turn on" => Action::TurnOn,
            "toggle" => Action::Toggle,
            "turn off" => Action::TurnOff,
            _ => panic!("Weird Shit: {:?}", value)
        }
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let splits: Vec<&str> = value
            .split(",")
            .filter(|x| !x.is_empty())
            .collect();

        Point {
            x: splits[0].parse::<i64>().unwrap(),
            y: splits[1].parse::<i64>().unwrap()
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    a: Action,
    s: Point,
    e: Point
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        todo!()
    }
}

fn parse_command(input: &str) -> Vec<String> {
    let re = Regex::new(r"(.+?) (\d+,\d+) through (\d+,\d+)").unwrap();
    let captures = re.captures(input).unwrap();

    vec![
        captures[1].to_string(), // The command, e.g., "toggle"
        captures[2].to_string(), // The first coordinate pair, e.g., "753,664"
        captures[3].to_string(), // The second coordinate pair, e.g., "970,926"
    ]
}


fn read_input(p: &str) -> Vec<Instruction> {
    let mut f = File::open(p).unwrap();
    let mut s: String = String::new();

    f.read_to_string(&mut s).unwrap();

    let i: Vec<Vec<String>> = s
        .trim()
        .split('\n')
        .map(|x| parse_command(x.trim()))
        .collect();

    let mut instructions: Vec<Instruction> = Vec::new();

    for x in i {
        instructions.push(
            Instruction {
                a: Action::from(x[0].as_str()),
                s: Point::from(x[1].as_str()),
                e: Point::from(x[2].as_str())
            }
        )
    }

    return instructions
}

fn main() {
    let input = read_input("../input");

    let mut m: Grid = Grid {
        mat: Mat::zeros(1000, 1000)
    };

    for x in input.clone() {
        m.follow_instruction(x)
    }

    let ans1 = m.mat
        .col_iter()
        .map(|x| x.sum())
        .reduce(|acc, e| acc + e)
        .unwrap();

    println!("Problem 1: {:?}", ans1 as i64);

    m.mat = Mat::zeros(1000, 1000);

    for x in input {
        m.follow_nordic_instruction(x)
    }

    let ans2 = m.mat
        .col_iter()
        .map(|x| x.sum())
        .reduce(|acc, e| acc + e)
        .unwrap();

    println!("Problem 2: {:?}", ans2 as i64);
}
