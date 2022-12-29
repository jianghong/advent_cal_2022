use std::fs::File;
use core::fmt;
use std::iter::Enumerate;
use std::io::Lines;
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

struct Job {
    instruction: Instruction,
    cycles_done: i64,
}

struct CPU {
    register_x: Register,
    cycle_count: i64,
    render_position: i64,
    next_line_break_check: i64,
    instructions: Enumerate<Lines<BufReader<File>>>,
    current_job: Option<Job>,
}

impl CPU {
    const FINAL_CYCLE: i64 = 240;
    const LINE_BREAK_INCREMENT: i64 = 40;

    fn new(instructions_filename: String) -> CPU {
        let file = File::open(instructions_filename).unwrap();
        let reader = BufReader::new(file);
        CPU {
            register_x: Register { value: 1 },
            cycle_count: 1,
            render_position: 0,
            next_line_break_check: 40,
            instructions: reader.lines().enumerate(),
            current_job: None,
        }
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

    fn run(&mut self) {
        while self.cycle_count <= CPU::FINAL_CYCLE {
            self.queue_instruction();
            self.render();
            self.work_on_current_job();
            self.cycle_count += 1;
        }
        println!("");
    }

    fn work_on_current_job(&mut self) {
        if let Some(job) = &mut self.current_job {
            job.cycles_done += 1;
            if job.cycles_done == job.instruction.cycles_needed {
                match job.instruction.instruction_type {
                    InstructionType::NOOP => {},
                    InstructionType::ADDX => self.register_x.value += job.instruction.operand.as_ref().unwrap().parse::<i64>().unwrap(),
                }
                self.current_job = None;
            }
        }
    }

    fn queue_instruction(&mut self) {
        if self.current_job.is_some() {
            return;
        }
        let next_instruction = self.instructions.next();

        if let Some((_, Ok(line))) = next_instruction {
            self.queue_job(CPU::parse_instruction(line));
        }
    }

    fn queue_job(&mut self, instruction: Instruction) {
        self.current_job = Some(Job {
            instruction,
            cycles_done: 0,
        });
    }

    fn render(&mut self) {
        self.line_break();
        self.render_sprite();
    }
    
    fn line_break(&mut self) {
        if self.cycle_count > self.next_line_break_check {
            println!("");
            self.next_line_break_check += CPU::LINE_BREAK_INCREMENT;
        }
    }

    fn render_sprite(&mut self) {
        // println!("cycle count: {}, register value: {}", self.cycle_count, self.register_x.value);
        let sprite_locations = vec![self.register_x.value - 1, self.register_x.value, self.register_x.value + 1];
        if sprite_locations.contains(&self.render_position) {
            print!("#");
        } else {
            print!(".");
        }
        self.render_position = (self.render_position + 1) % CPU::LINE_BREAK_INCREMENT;
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {}, cycles: {}", self.register_x.value, self.cycle_count)
    }
}

fn main() {
    let mut cpu = CPU::new("input.txt".to_string());
    cpu.run();
}
