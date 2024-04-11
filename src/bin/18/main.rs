use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

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
    Snd(char),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        let reg = split[1].chars().next().unwrap();
        match split[0] {
            "snd" => Ok(Instruction::Snd(reg)),
            "set" => Ok(Instruction::Set(reg, split[2].parse::<Value>().unwrap())),
            "add" => Ok(Instruction::Add(reg, split[2].parse::<Value>().unwrap())),
            "mul" => Ok(Instruction::Mul(reg, split[2].parse::<Value>().unwrap())),
            "mod" => Ok(Instruction::Mod(reg, split[2].parse::<Value>().unwrap())),
            "rcv" => Ok(Instruction::Rcv(reg)),
            "jgz" => Ok(Instruction::Jgz(
                split[1].parse::<Value>().unwrap(),
                split[2].parse::<Value>().unwrap(),
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
            Instruction::Add(reg, val) => {
                let reg_val = registers.get(reg).unwrap_or(&0);
                registers.insert(*reg, reg_val + val.resolve(registers));
            }
            Instruction::Mul(reg, val) => {
                let reg_val = registers.get(reg).unwrap_or(&0);
                registers.insert(*reg, reg_val * val.resolve(registers));
            }
            Instruction::Mod(reg, val) => {
                let reg_val = registers.get(reg).unwrap_or(&0);
                registers.insert(*reg, reg_val % val.resolve(registers));
            }
            Instruction::Snd(_) => (),
            Instruction::Rcv(_) => (),
            Instruction::Jgz(_, _) => (),
        }
    }
}

struct Program {
    registers: HashMap<char, i64>,
    instructions: Vec<Instruction>,
    pc: i64,
    last_sound: Option<i64>,
    queue: VecDeque<i64>,
    num_sends: u32,
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
            last_sound: None,
            queue: VecDeque::new(),
            num_sends: 0,
        })
    }
}

impl Program {
    fn step_until_rcv_p1(&mut self) -> i64 {
        if self.pc < 0 || self.pc >= self.instructions.len() as i64 {
            unreachable!("Program counter out of bounds");
        }

        let instruction = &self.instructions[self.pc as usize];
        instruction.eval(&mut self.registers);

        let mut jump_offset = 1;
        match instruction {
            Instruction::Rcv(val) => {
                if *self.registers.get(val).unwrap_or(&0) != 0 {
                    return self.last_sound.expect("No sound played before rcv");
                }
            }
            Instruction::Snd(reg) => self.last_sound = Some(*self.registers.get(reg).unwrap_or(&0)),
            Instruction::Jgz(cmp, offset) => {
                if cmp.resolve(&self.registers) > 0 {
                    jump_offset = offset.resolve(&self.registers);
                }
            }
            _ => (),
        }

        self.pc += jump_offset;
        self.step_until_rcv_p1()
    }

    fn step_until_block_p2(&mut self, other: &mut Self) {
        if self.pc < 0 || self.pc >= self.instructions.len() as i64 {
            unreachable!("Program counter out of bounds");
        }

        let instruction = &self.instructions[self.pc as usize];
        instruction.eval(&mut self.registers);

        let mut jump_offset = 1;
        match instruction {
            Instruction::Rcv(reg) => {
                if let Some(val) = self.queue.pop_front() {
                    self.registers.insert(*reg, val);
                } else {
                    return;
                }
            }
            Instruction::Snd(reg) => {
                other
                    .queue
                    .push_back(*self.registers.get(reg).unwrap_or(&0));
                self.num_sends += 1;
            }
            Instruction::Jgz(cmp, offset) => {
                if cmp.resolve(&self.registers) > 0 {
                    jump_offset = offset.resolve(&self.registers);
                }
            }
            _ => (),
        }

        self.pc += jump_offset;
        self.step_until_block_p2(other)
    }
}

fn p1(input: &str) -> i64 {
    input.parse::<Program>().unwrap().step_until_rcv_p1()
}

fn p2(input: &str) -> i64 {
    let mut p0 = input.parse::<Program>().unwrap();
    let mut p1 = input.parse::<Program>().unwrap();
    p0.registers.insert('p', 0);
    p1.registers.insert('p', 1);

    loop {
        p0.step_until_block_p2(&mut p1);
        p1.step_until_block_p2(&mut p0);
        if p0.queue.is_empty() && p1.queue.is_empty() {
            break;
        }
    }

    p1.num_sends as i64
}

fn main() {
    run_day(p1, p2);
}
