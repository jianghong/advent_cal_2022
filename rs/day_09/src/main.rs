use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Segment {
    next: Option<Box<Segment>>,
    pos: Position,
}

impl Segment {
    fn is_tail(&self) -> bool {
        self.next.is_none()
    }
}

#[derive(Debug)]
struct Rope {
    head: Option<Box<Segment>>
}

impl Rope {
    fn new(size: u64) -> Rope {
        let mut rope = Rope {
            head: None,
        };
        rope.push_n(size);
        rope
    }

    fn push_n(&mut self, n: u64) {
        for _ in 0..n {
            let new_segment = Box::new(Segment {
                next: self.head.take(),
                pos: Position { x: 0, y: 0 },
            });
            self.head = Some(new_segment);
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

struct KnotTracker {
    known_tail_positions: Vec<Position>,
    rope: Rope,
}

impl KnotTracker {
    fn new() -> KnotTracker {
        KnotTracker {
            known_tail_positions: vec![Position { x: 0, y: 0 }],
            rope: Rope::new(10)
        }
    }

    fn add_current_tail_position(&mut self, pos: Position) {
        if !self.known_tail_positions.contains(&pos) {
            self.known_tail_positions.push(pos);
        }
    }

    fn move_head(&mut self, direction: &str, steps: i64) {
        match direction {
            "R" => self.move_head_right(steps),
            // "L" => self.move_head_left(steps),
            // "U" => self.move_head_up(steps),
            // "D" => self.move_head_down(steps),
            _ => panic!("Invalid direction"),
        }
    }

    fn move_head_right(&mut self, steps: i64) {
        for _ in 0..steps {
            let mut curr_segment = self.rope.head.as_mut().unwrap();
            curr_segment.pos.x += 1;
            
            while curr_segment.next.is_some() {
                let next_segment = curr_segment.next.as_mut().unwrap();
                let dist = distance_of_curr_and_next(&curr_segment.pos, &next_segment.pos);
                
                if dist.0 == 2 {
                    next_segment.pos.x += 1;
                    next_segment.pos.y += dist.1;
                }

                curr_segment = next_segment;
            }
        }
    }

}

fn distance_of_curr_and_next(curr_pos: &Position, next_pos: &Position) -> (i64, i64) {
    let x_diff = curr_pos.x - next_pos.x;
    let y_diff = curr_pos.y - next_pos.y;

    (x_diff, y_diff)
}

fn main() {
    let file = std::fs::File::open("sample.txt").unwrap();
    let reader = BufReader::new(file);

    let mut knot_tracker = KnotTracker::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        let direction = words.next().unwrap();
        let steps = words.next().unwrap().parse::<i64>().unwrap();
        
        knot_tracker.move_head(direction, steps);
    }

    println!("{:?}", knot_tracker.rope);
}
