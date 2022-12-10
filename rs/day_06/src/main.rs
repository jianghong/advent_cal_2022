use std::process::exit;

struct FourItemStack {
    stack: [char; 4]
}

impl FourItemStack {
    fn new() -> FourItemStack {
        FourItemStack {
            stack: [' ', ' ', ' ', ' ']
        }
    }

    fn push(&mut self, c: char) {
        self.stack[0] = self.stack[1];
        self.stack[1] = self.stack[2];
        self.stack[2] = self.stack[3];
        self.stack[3] = c;
    }

    fn has_duplicates(&self) -> bool {
        self.stack[0] == self.stack[1] || self.stack[0] == self.stack[2] || self.stack[0] == self.stack[3] ||
        self.stack[1] == self.stack[2] || self.stack[1] == self.stack[3] ||
        self.stack[2] == self.stack[3]
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].clone();
    let input = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut four_stack = FourItemStack::new();
    
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
