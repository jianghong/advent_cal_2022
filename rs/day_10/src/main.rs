use core::fmt;
use std::io::{BufRead, BufReader};

struct Register {
    value: i64
}

#[derive(PartialEq)]
enum InstructionType {
    NOOP,
    ADDX
}

struct Instruction {
    instruction_type: InstructionType,
    operand: Option<String>
}

impl Instruction {
    fn new(instruction_type: InstructionType, operand: Option<String>) -> Instruction {
        if instruction_type == InstructionType::ADDX && operand.is_none() {
            panic!("ADDX instruction must have an operand");
        }
        Instruction {
            instruction_type,
            operand
        }
    }
}

struct CPU {
    register_x: Register,
    cycle_count: i64,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            register_x: Register { value: 1 },
            cycle_count: 0
        }
    }

    fn handle_instruction(&mut self, instruction: Instruction) {
        match instruction.instruction_type {
            InstructionType::NOOP => self.handle_noop(),
            InstructionType::ADDX => self.handle_addx(instruction.operand.unwrap().parse::<i64>().unwrap()),
        }
    }

    fn handle_noop(&mut self) {
        self.cycle_count += 1;
    }

    fn handle_addx(&mut self, operand: i64) {
        self.register_x.value += operand;
        self.cycle_count += 2;
    }

    fn parse_instruction(instruction: String) -> Instruction {
        // match on "noop" or "addx 5"
        match instruction.as_str() {
            "noop" => Instruction::new(InstructionType::NOOP, None),
            addx_instruction if addx_instruction.starts_with("addx") => {
                let operand = addx_instruction.split(" ").nth(1).unwrap();
                Instruction::new(InstructionType::ADDX, Some(operand.to_string()))
            },
            _ => panic!("Unknown instruction")
        }

    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {}, cycles: {}", self.register_x.value, self.cycle_count)
    }
}

fn main() {
    let file = std::fs::File::open("sample.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut cpu = CPU::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let instruction = CPU::parse_instruction(line);
        cpu.handle_instruction(instruction);
    }

    println!("{}", cpu);
}
