use std::io::{BufReader, BufRead};

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

struct KnotTracker {
    curr_head_pos: Position,
    curr_tail_pos: Position,
    known_tail_positions: Vec<Position>,
}

impl KnotTracker {
    fn new() -> KnotTracker {
        KnotTracker {
            curr_head_pos: Position { x: 0, y: 0 },
            curr_tail_pos: Position { x: 0, y: 0 },
            known_tail_positions: vec![Position { x: 0, y: 0 }]
        }
    }

    fn add_current_tail_position(&mut self) {
        if !self.known_tail_positions.contains(&self.curr_tail_pos) {
            self.known_tail_positions.push(self.curr_tail_pos);
        }
    }

    fn move_head(&mut self, direction: &str, steps: i64) {
        match direction {
            "R" => self.move_head_right(steps),
            "L" => self.move_head_left(steps),
            "U" => self.move_head_up(steps),
            "D" => self.move_head_down(steps),
            _ => panic!("Invalid direction"),
        }

        println!("Current head position: {:?}", self.curr_head_pos);
        println!("Current tail position: {:?}", self.curr_tail_pos);
    }

    fn move_head_right(&mut self, steps: i64) {
        for _ in 0..steps {
            self.curr_head_pos = Position { x: self.curr_head_pos.x + 1, y: self.curr_head_pos.y };
            let dist = self.distance_of_head_and_tail(); 
            
            if dist.0 == 2 {
                self.curr_tail_pos = Position {
                    x: self.curr_tail_pos.x + 1,
                    y: self.curr_tail_pos.y + dist.1
                };
                self.add_current_tail_position();
            }
        }
    }

    fn move_head_up(&mut self, steps: i64) {
        for _ in 0..steps {
            self.curr_head_pos = Position { x: self.curr_head_pos.x, y: self.curr_head_pos.y + 1 };
            let dist = self.distance_of_head_and_tail(); 

            if dist.1 == 2 {
                self.curr_tail_pos = Position {
                    x: self.curr_tail_pos.x + dist.0,
                    y: self.curr_tail_pos.y + 1
                };
                self.add_current_tail_position();
            }
        }
    }

    fn move_head_left(&mut self, steps: i64) {
        for _ in 0..steps {
            self.curr_head_pos = Position { x: self.curr_head_pos.x - 1, y: self.curr_head_pos.y };
            let dist = self.distance_of_head_and_tail(); 

            if dist.0 == -2 {
                self.curr_tail_pos = Position {
                    x: self.curr_tail_pos.x - 1,
                    y: self.curr_tail_pos.y + dist.1
                };
                self.add_current_tail_position();
            }
        } 
    }

    fn move_head_down(&mut self, steps: i64) {
        for _ in 0..steps {
            self.curr_head_pos = Position { x: self.curr_head_pos.x, y: self.curr_head_pos.y - 1 };
            let dist = self.distance_of_head_and_tail(); 

            if dist.1 == -2 {
                self.curr_tail_pos = Position {
                    x: self.curr_tail_pos.x + dist.0,
                    y: self.curr_tail_pos.y - 1
                };
                self.add_current_tail_position();
            }
        }
    }

    fn distance_of_head_and_tail(&self) -> (i64, i64) {
        let x_diff = self.curr_head_pos.x - self.curr_tail_pos.x;
        let y_diff = self.curr_head_pos.y - self.curr_tail_pos.y;

        (x_diff, y_diff)
    }
}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut knot_tracker = KnotTracker::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        let direction = words.next().unwrap();
        let steps = words.next().unwrap().parse::<i64>().unwrap();
        
        knot_tracker.move_head(direction, steps);
    }

    println!("Known tail positions: {:?}", knot_tracker.known_tail_positions);
    println!("num tail positions: {}", knot_tracker.known_tail_positions.len());
}
