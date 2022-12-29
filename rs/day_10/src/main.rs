use core::fmt;
use std::io::{BufRead, BufReader};

struct Register {
    value: i64
}

#[derive(Debug, PartialEq)]
enum InstructionType {
    NOOP,
    ADDX
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    cycles_needed: i64,
    operand: Option<String>,
}

impl Instruction {
    fn new(instruction_type: InstructionType, cycles_needed: i64, operand: Option<String>) -> Instruction {
        if instruction_type == InstructionType::ADDX && operand.is_none() {
            panic!("ADDX instruction must have an operand");
        }
        Instruction {
            instruction_type,
            cycles_needed,
            operand
        }
    }
}

struct CPU {
    register_x: Register,
    cycle_count: i64,
    signal_strength: i64,
    next_cycle_count_check: i64,
}

impl CPU {
    const FINAL_CYCLE_COUNT_CHECK: i64 = 220;
    const CYCLE_CHECK_INCREMENT: i64 = 40;

    fn new() -> CPU {
        CPU {
            register_x: Register { value: 1 },
            cycle_count: 0,
            signal_strength: 0,
            next_cycle_count_check: 20
        }
    }

    fn handle_instruction(&mut self, instruction: Instruction) {
        self.cycle_count += instruction.cycles_needed;
        self.add_signal_strength();
        match instruction.instruction_type {
            InstructionType::NOOP => self.handle_noop(),
            InstructionType::ADDX => self.handle_addx(instruction.operand.unwrap().parse::<i64>().unwrap()),
        }
    }

    fn handle_noop(&mut self) {
    }

    fn handle_addx(&mut self, operand: i64) {
        self.register_x.value += operand;
    }

    fn add_signal_strength(&mut self) {
        if self.next_cycle_count_check > CPU::FINAL_CYCLE_COUNT_CHECK {
            return
        }

        if self.cycle_count >= self.next_cycle_count_check {
            self.signal_strength += self.register_x.value * self.next_cycle_count_check;
            self.next_cycle_count_check += CPU::CYCLE_CHECK_INCREMENT;
        }
        println!("cycle count: {}, next cycle count check: {}, signal strength {}, register x {}", self.cycle_count, self.next_cycle_count_check, self.signal_strength, self.register_x.value);
    }

    fn parse_instruction(instruction: String) -> Instruction {
        match instruction.as_str() {
            "noop" => Instruction::new(InstructionType::NOOP, 1, None),
            addx_instruction if addx_instruction.starts_with("addx") => {
                let operand = addx_instruction.split(" ").nth(1).unwrap();
                Instruction::new(InstructionType::ADDX, 2, Some(operand.to_string()))
            },
            _ => panic!("Unknown instruction")
        }

    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {}, cycles: {}, signal strength: {}", self.register_x.value, self.cycle_count, self.signal_strength)
    }
}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut cpu = CPU::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let instruction = CPU::parse_instruction(line);
        cpu.handle_instruction(instruction);
    }

    println!("{}", cpu);
}
