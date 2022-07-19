use std::collections::HashMap;

#[derive(Clone)]
enum Value {
    Integer(isize),
    Register(char),
}

impl Value {
    fn get(&self, registers: &HashMap<char, isize>) -> isize {
        match self {
            Value::Integer(i) => *i,
            Value::Register(r) => *registers.get(r).unwrap(),
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Inp(char),
    Add(char, Value),
    Mul(char, Value),
    Div(char, Value),
    Mod(char, Value),
    Eql(char, Value),
}

fn compute(instruction: &Instruction, registers: &mut HashMap<char, isize>, next_input: usize) {
    match instruction {
        Instruction::Inp(c) => registers.insert(*c, next_input as isize),
        Instruction::Add(c, v) => {
            registers.insert(*c, registers.get(c).unwrap() + v.get(&registers))
        }
        Instruction::Mul(c, v) => {
            registers.insert(*c, registers.get(c).unwrap() * v.get(&registers))
        }
        Instruction::Div(c, v) => {
            registers.insert(*c, registers.get(c).unwrap() / v.get(&registers))
        }
        Instruction::Mod(c, v) => {
            registers.insert(*c, registers.get(c).unwrap() % v.get(&registers))
        }
        Instruction::Eql(c, v) => registers.insert(
            *c,
            if *registers.get(c).unwrap() == v.get(&registers) {
                1
            } else {
                0
            },
        ),
    };
}

pub fn task1(input: &str) {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let first = parts[1].chars().next().unwrap();
        let second = if parts[0] != "inp" {
            if parts[2].len() == 1 && parts[2].chars().next().unwrap().is_alphabetic() {
                Some(Value::Register(parts[2].chars().next().unwrap()))
            } else {
                Some(Value::Integer(parts[2].parse().unwrap()))
            }
        } else {
            None
        };
        instructions.push(match parts[0] {
            "inp" => Instruction::Inp(first),
            "add" => Instruction::Add(first, second.unwrap()),
            "mul" => Instruction::Mul(first, second.unwrap()),
            "div" => Instruction::Div(first, second.unwrap()),
            "mod" => Instruction::Mod(first, second.unwrap()),
            "eql" => Instruction::Eql(first, second.unwrap()),
            _ => panic!(),
        })
    }

    let mut instruction_parts = Vec::new();
    let mut last_start = 0;
    for i in 1..instructions.len() {
        match instructions[i] {
            Instruction::Inp(_) => {
                instruction_parts.push(instructions[last_start..i].to_vec());
                last_start = i;
            }
            _ => (),
        }
    }
    instruction_parts.push(instructions[last_start..].to_vec());

    let mut results: HashMap<isize, (usize, usize)> = HashMap::new(); // (z, (lowest, highest))
    results.insert(0, (0, 0));

    for part in 0..14 {
        println!("part={}: {}", part, results.len());
        let mut new_results: HashMap<isize, (usize, usize)> = HashMap::new();
        for entry in results.iter() {
            for digit in 1..=9 {
                let mut registers = HashMap::new();
                registers.insert('w', 0);
                registers.insert('x', 0);
                registers.insert('y', 0);
                registers.insert('z', *entry.0);
                for instruction in instruction_parts[part].iter() {
                    compute(instruction, &mut registers, digit);
                }

                let z = *registers.get(&'z').unwrap();
                let new_low_num = entry.1 .0 * 10 + digit;
                let new_high_num = entry.1 .1 * 10 + digit;
                if let Some(existing) = new_results.get(&z).cloned() {
                    new_results.insert(
                        z,
                        (new_low_num.min(existing.0), new_high_num.max(existing.1)),
                    );
                } else {
                    new_results.insert(z, (new_low_num, new_high_num));
                }
            }
        }
        results = new_results;
    }
    println!("Result: {:?}", results.get(&0));
}

pub fn task2(input: &str) {
    let mut result = 0usize;
    println!("Result: {}", result);
}
