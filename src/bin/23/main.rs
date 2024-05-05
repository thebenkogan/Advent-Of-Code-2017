use std::{collections::HashMap, str::FromStr};

use aoc2017::run_day;

enum Value {
    Register(char),
    Value(i64),
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i64>() {
            Ok(Value::Value(n))
        } else {
            Ok(Value::Register(s.chars().next().unwrap()))
        }
    }
}

impl Value {
    fn resolve(&self, registers: &HashMap<char, i64>) -> i64 {
        match self {
            Value::Register(r) => *registers.get(r).unwrap_or(&0),
            Value::Value(v) => *v,
        }
    }
}

enum Instruction {
    Set(char, Value),
    Mul(char, Value),
    Sub(char, Value),
    Jnz(Value, i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        let reg = split[1].chars().next().unwrap();
        match split[0] {
            "set" => Ok(Instruction::Set(reg, split[2].parse::<Value>().unwrap())),
            "mul" => Ok(Instruction::Mul(reg, split[2].parse::<Value>().unwrap())),
            "sub" => Ok(Instruction::Sub(reg, split[2].parse::<Value>().unwrap())),
            "jnz" => Ok(Instruction::Jnz(
                split[1].parse::<Value>().unwrap(),
                split[2].parse::<i32>().unwrap(),
            )),
            _ => Err(format!("Invalid instruction: {}", s)),
        }
    }
}

impl Instruction {
    fn eval(&self, registers: &mut HashMap<char, i64>) {
        match self {
            Instruction::Set(reg, val) => {
                registers.insert(*reg, val.resolve(registers));
            }
            Instruction::Mul(reg, val) => {
                let reg_val = registers.get(reg).unwrap_or(&0);
                registers.insert(*reg, reg_val * val.resolve(registers));
            }
            Instruction::Sub(reg, val) => {
                let reg_val = registers.get(reg).unwrap_or(&0);
                registers.insert(*reg, reg_val - val.resolve(registers));
            }
            Instruction::Jnz(_, _) => {}
        }
    }
}

struct Program {
    registers: HashMap<char, i64>,
    instructions: Vec<Instruction>,
    pc: i64,
    num_mul: u64,
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> = s
            .lines()
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect();
        Ok(Program {
            registers: HashMap::new(),
            instructions,
            pc: 0,
            num_mul: 0,
        })
    }
}

impl Program {
    fn step_until_end(&mut self) -> u64 {
        loop {
            if self.pc < 0 || self.pc >= self.instructions.len() as i64 {
                return self.num_mul;
            }

            let instruction = &self.instructions[self.pc as usize];
            instruction.eval(&mut self.registers);

            let mut jump_offset = 1;
            match instruction {
                Instruction::Jnz(cmp, offset) => {
                    if cmp.resolve(&self.registers) != 0 {
                        jump_offset = *offset;
                    }
                }
                Instruction::Mul(_, _) => {
                    self.num_mul += 1;
                }
                _ => {}
            }

            self.pc += jump_offset as i64;
        }
    }
}

fn p1(input: &str) -> i32 {
    let mut program = input.parse::<Program>().unwrap();
    program.step_until_end() as i32
}

fn p2(input: &str) -> i32 {
    let mut program = input.parse::<Program>().unwrap();
    program.instructions[0].eval(&mut program.registers);
    let start = program.registers.get(&'b').unwrap_or(&0);

    let mut nonprimes = 0;
    let x = start * 100 + 100000;
    for n in (x..=x + 17000).step_by(17) {
        let mut d = 2;
        while n % d != 0 {
            d += 1;
        }
        if n != d {
            nonprimes += 1;
        }
    }
    nonprimes
}

fn main() {
    run_day(p1, p2);
}
