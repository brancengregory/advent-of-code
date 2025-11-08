use std::fs::read_to_string;
use std::fmt::Debug;
use regex::Regex;


#[derive(Clone)]
struct Grid {
    lights: Vec<u32>,
    width: usize
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            lights: vec![0; width * height],
            width
        }
    }

    fn apply(&mut self, start: &Point, end: &Point, f: impl Fn(u32) -> u32) {
        for y in start.y..=end.y {
            for x in start.x..=end.x {
                let index = (y as usize) * self.width + (x as usize);
                self.lights[index] = f(self.lights[index]);
            }
        }
    }
    
    fn turn_on(&mut self, start: &Point, end: &Point) {
        self.apply(start, end, |_| 1);
    }

    fn turn_off(&mut self, start: &Point, end: &Point) {
        self.apply(start, end, |_| 0);
    }

    fn toggle(&mut self, start: &Point, end: &Point) {
        self.apply(start, end, |v| 1 - v);
    }

    fn nordic_turn_on(&mut self, start: &Point, end: &Point) {
        self.apply(start, end, |v| v + 1);
    }

    fn nordic_turn_off(&mut self, start: &Point, end: &Point) {
        self.apply(start, end, |v| v.saturating_sub(1));
    }

    fn nordic_toggle(&mut self, start: &Point, end: &Point) {
        self.apply(start, end, |v| v + 2);
    }

    fn follow_instruction(&mut self, i: &Instruction) {
        match i.a {
            Action::TurnOn => self.turn_on(&i.s, &i.e),
            Action::Toggle => self.toggle(&i.s, &i.e),
            Action::TurnOff => self.turn_off(&i.s, &i.e)
        };
    }

    fn follow_nordic_instruction(&mut self, i: &Instruction) {
        match i.a {
            Action::TurnOn => self.nordic_turn_on(&i.s, &i.e),
            Action::Toggle => self.nordic_toggle(&i.s, &i.e),
            Action::TurnOff => self.nordic_turn_off(&i.s, &i.e)
        };
    }

    fn sum(&self) -> u32 {
        self.lights.iter().sum()
    }

    fn reset(&mut self) {
        self.lights.fill(0);
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

fn read_input(p: &str) -> Vec<Instruction> {
    read_to_string(p) 
        .unwrap()
        .lines()
        .map(|x| {
            let re = Regex::new(r"(.+?) (\d+,\d+) through (\d+,\d+)").unwrap();
            let captures = re.captures(x).unwrap();

            Instruction {
                a: Action::from(&captures[1]),
                s: Point::from(&captures[2]),
                e: Point::from(&captures[3])
            }
        })
        .collect()
}

fn main() {
    let input = read_input("../input");

    let mut m: Grid = Grid::new(1000, 1000);

    for i in input.iter() {
        m.follow_instruction(i);
    }

    println!("Problem 1: {}", m.sum());

    m.reset();

    for i in input.iter() {
        m.follow_nordic_instruction(i);
    }

    println!("Problem 2: {}", m.sum());
}
