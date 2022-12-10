use std::process::exit;

struct NItemStack {
    stack: Vec<char>,
    n: usize
}

impl NItemStack {
    fn new(n: usize) -> NItemStack {
        NItemStack {
            stack: vec![' '; n],
            n: n
        }
    }

    fn push(&mut self, c: char) {
        for i in 0..self.n-1 {
            self.stack[i] = self.stack[i+1];
        }
        self.stack[self.n-1] = c;
    }

    fn has_duplicates(&self) -> bool {
        for i in 0..self.n-1 {
            for j in i+1..self.n {
                if self.stack[i] == self.stack[j] {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].clone();

    part2(filename.clone());
}

fn part2(filename: String) {
    let input = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut fourteen_stack = NItemStack::new(14);
    for (i, c) in input.chars().enumerate() {
        fourteen_stack.push(c);
        if (i > 2) && (!fourteen_stack.has_duplicates()) {
            println!("Found start-of-message marker at the {}th character", i + 1);
            exit(0);
        }
    }
    println!("No start-of-message marker found");
}

fn part1(filename: String) {
    let input = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut four_stack = NItemStack::new(4);
    
    // iterate input with index
    for (i, c) in input.chars().enumerate() {
        four_stack.push(c);
        if (i > 2) && (!four_stack.has_duplicates()) {
            println!("Found start-of-packet marker at the {}th character", i + 1);
            exit(0);
        }
    }
    println!("No start-of-packet marker found");
}