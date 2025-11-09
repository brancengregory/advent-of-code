use std::{collections::HashMap, fs::File, io::Read};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Operand {
    Signal(u16),
    Wire(String)
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        value.parse::<u16>()
            .map(Operand::Signal)
            .unwrap_or_else(|_| Operand::Wire(value.to_string()))
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Assign(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, Operand),
    RShift(Operand, Operand)
}

#[derive(Debug, Clone)]
struct Instruction {
    operation: Operation,
    target: String
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let splits: Vec<&str> = s
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .collect();

        let i = match splits.len() {
            3 => {
                let operands = vec![Operand::from(splits[0])];
                let target = splits[2].to_string();
                Instruction {
                    operator: None,
                    operands: Some(operands),
                    target
                }
            },
            4 => {
                let operator = Operator::from(splits[0]);
                let operands = vec![Operand::from(splits[1])];
                let target = splits[3].to_string();
                Instruction {
                    operator: Some(operator),
                    operands: Some(operands),
                    target
                }
            },
            5 => {
                let operator = Operator::from(splits[1]);
                let operands = vec![
                    Operand::from(splits[0]),
                    Operand::from(splits[2])
                ];
                let target = splits[4].to_string();
                Instruction {
                    operator: Some(operator),
                    operands: Some(operands),
                    target
                }
            },
            _ => panic!()
        };

        return i
    }
}

impl Instruction {
    fn evaluate(&self, cache: &mut HashMap<String, u16>, instructions: &HashMap<String, Instruction>) -> u16 {
        if let Some(&value) = cache.get(&self.target) {
            return value
        }

        let value = if let Some(operator) = &self.operator {
            let operands = self.operands.as_ref().unwrap();
            match operator {
                Operator::And => {
                    let left = operands[0].evaluate(cache, instructions);
                    let right = operands[1].evaluate(cache, instructions);
                    left & right
                }
                Operator::Or => {
                    let left = operands[0].evaluate(cache, instructions);
                    let right = operands[1].evaluate(cache, instructions);
                    left | right
                }
                Operator::Not => {
                    let operand = operands[0].evaluate(cache, instructions);
                    !operand
                }
                Operator::LShift => {
                    let left = operands[0].evaluate(cache, instructions);
                    let right = operands[1].evaluate(cache, instructions);
                    left << right
                }
                Operator::RShift => {
                    let left = operands[0].evaluate(cache, instructions);
                    let right = operands[1].evaluate(cache, instructions);
                    left >> right
                }
            }
        } else {
            self.operands.as_ref().unwrap_or_else(|| {
                panic!("No operand found in instruction: {:?}", self)
            })[0].evaluate(cache, instructions)
        };

        cache.insert(self.target.clone(), value);
        return value
    }
}

impl Operand {
    fn evaluate(&self, cache: &mut HashMap<String, u16>, instructions: &HashMap<String, Instruction>) -> u16 {
        match self {
            Operand::Signal(value) => *value,
            Operand::Wire(wire) => {
                if let Some(&value) = cache.get(wire) {
                    return value;
                }

                let instruction = instructions.get(wire).unwrap_or_else(|| {
                    panic!("No instruction found for wire: {}", wire);
                });
                instruction.evaluate(cache, instructions)
            }
        }
    }
}

fn read_instructions(p: &str) -> HashMap<String, Instruction> {
    let mut f = File::open(p).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let i = Instruction::from(x);
            (i.target.clone(), i)
        })
        .collect()
}

fn main() {
    let instructions = read_instructions("../input");
    let mut result_cache: HashMap<String, u16> = HashMap::new();
    let mut ans1: u16 = 0;

    if let Some(instruction) = instructions.get("a") {
        ans1 = instruction.evaluate(&mut result_cache, &instructions);
        println!("Problem 1: {:?}", ans1)
    } else {
        println!("No instructions found")
    }

    result_cache.clear();
    result_cache.insert("b".to_string(), ans1);

    let ans2 = instructions.get("a").unwrap_or_else(|| panic!("No instruction found"))
        .evaluate(&mut result_cache, &instructions);

    println!("Problem 2: {:?}", ans2);
}
