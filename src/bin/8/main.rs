use std::{collections::HashMap, str::FromStr};

use aoc2017::run_day;

enum Binop {
    Greater,
    Less,
    GreaterEq,
    LessEq,
    Eq,
    NotEq,
}

impl FromStr for Binop {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Binop::Greater),
            "<" => Ok(Binop::Less),
            ">=" => Ok(Binop::GreaterEq),
            "<=" => Ok(Binop::LessEq),
            "==" => Ok(Binop::Eq),
            "!=" => Ok(Binop::NotEq),
            _ => Err(format!("Invalid binop: {}", s)),
        }
    }
}
struct CondExpr {
    reg: String,
    binop: Binop,
    val: i32,
}

impl CondExpr {
    fn new(reg: String, binop: Binop, val: i32) -> Self {
        Self { reg, binop, val }
    }

    fn eval(&self, registers: &HashMap<String, i32>) -> bool {
        let reg_val = registers.get(&self.reg).unwrap_or(&0);
        match self.binop {
            Binop::Greater => reg_val > &self.val,
            Binop::Less => reg_val < &self.val,
            Binop::GreaterEq => reg_val >= &self.val,
            Binop::LessEq => reg_val <= &self.val,
            Binop::Eq => reg_val == &self.val,
            Binop::NotEq => reg_val != &self.val,
        }
    }
}

fn eval_instructions(input: &str) -> (HashMap<String, i32>, i32) {
    let mut registers = HashMap::new();
    let mut max_val = 0;

    input.split('\n').for_each(|line| {
        let split: Vec<&str> = line.split(' ').collect();
        let reg = split[0];
        let sign = if split[1] == "inc" { 1 } else { -1 };
        let amnt = split[2].parse::<i32>().unwrap();
        let cond_reg = split[4];
        let binop = split[5].parse::<Binop>().unwrap();
        let cond_val = split[6].parse::<i32>().unwrap();
        let cond = CondExpr::new(cond_reg.to_string(), binop, cond_val);
        if cond.eval(&registers) {
            let reg_val = registers.get(reg).unwrap_or(&0);
            let new_val = reg_val + sign * amnt;
            max_val = max_val.max(new_val);
            registers.insert(reg.to_string(), new_val);
        }
    });

    (registers, max_val)
}

fn p1(input: &str) -> i32 {
    let (registers, _) = eval_instructions(input);
    registers.values().copied().max().unwrap()
}

fn p2(input: &str) -> i32 {
    let (_, max_val) = eval_instructions(input);
    max_val
}

fn main() {
    run_day(p1, p2);
}
