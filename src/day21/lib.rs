use std::collections::HashMap;
use itertools::Itertools;


#[derive(Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Operation {
    fn parse(operation: &str) -> Operation {
        match operation {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => unreachable!()
        }
    }

    pub fn execute(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
        }
    }
}

#[derive(Clone, Copy)]
pub struct OperationMonkey<'a> {
    pub operation: Operation,
    pub left: &'a str,
    pub right: &'a str
}

#[derive(Clone, Copy)]
pub struct ConstantMonkey {
    pub value: i64
}

#[derive(Clone, Copy)]
pub enum Monkey<'a> {
    Operation(OperationMonkey<'a>),
    Constant(ConstantMonkey)
}

pub fn parse_monkeys(input: &str) -> HashMap<&str, Monkey> {
    let mut lookup = HashMap::new();

    input
        .lines()
        .for_each(|line| {
            let parts = line.split(' ').collect_vec();
            let name = &parts[0][0..4];
            if parts.len() == 2 {
                // Constant
                let value = parts[1].parse().unwrap();
                let monkey = ConstantMonkey { value };
                lookup.insert(name, Monkey::Constant(monkey));
            } else {
                // Operation
                let operation = Operation::parse(parts[2]);
                let left = parts[1];
                let right = parts[3];
                let monkey = OperationMonkey { operation, left, right };
                lookup.insert(name, Monkey::Operation(monkey));
            }
        }
    );

    lookup
}