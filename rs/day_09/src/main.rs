use std::fmt;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Segment {
    next: Option<Box<Segment>>,
    pos: Position,
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

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let offset_x = 20;
        let offset_y = 20;
        let mut grid = vec![vec![".".to_string(); 100]; 100];

        let mut curr_segment = self.head.as_ref().unwrap();
        grid[(curr_segment.pos.y + offset_y) as usize][(curr_segment.pos.x + offset_x) as usize] = "H".to_string();
        let mut id = 1;
        while curr_segment.next.is_some() {
            curr_segment = curr_segment.next.as_ref().unwrap();
            if grid[offset_y as usize][offset_x as usize] == ".".to_string() {
                grid[(curr_segment.pos.y + offset_y) as usize][(curr_segment.pos.x + offset_x) as usize] = id.to_string();
            }
            id += 1;
        }
        
        if grid[offset_y as usize][offset_x as usize] == ".".to_string() {
            grid[offset_y as usize][offset_x as usize] = "S".to_string();
        }

        grid.reverse();
        for row in grid {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

struct KnotTracker {
    known_tail_positions: Vec<Position>,
}

impl KnotTracker {
    fn new() -> KnotTracker {
        KnotTracker {
            known_tail_positions: vec![Position { x: 0, y: 0 }],
        }
    }

    fn add_current_tail_position(&mut self, pos: Position) {
        if !self.known_tail_positions.contains(&pos) {
            self.known_tail_positions.push(pos);
        }
    }

    fn move_head(&mut self, rope: &mut Rope, direction: &str, steps: i64) {
        for _ in 0..steps {
            let mut curr_segment = rope.head.as_mut().unwrap();
            match direction {
                "R" => curr_segment.pos.x += 1,
                "L" => curr_segment.pos.x -=1,
                "U" => curr_segment.pos.y += 1,
                "D" => curr_segment.pos.y -= 1,
                _ => panic!("Invalid direction"),
            }

            while curr_segment.next.is_some() {
                let next_segment = curr_segment.next.as_mut().unwrap();
                let dist = distance_of_curr_and_next(&curr_segment.pos, &next_segment.pos);
                
                if dist == (2, 0) {
                    next_segment.pos.x += 1
                } else if dist == (0, 2) {
                    next_segment.pos.y += 1
                } else if dist.0 + dist.1 >= 3 {
                    next_segment.pos.x += 1;
                    next_segment.pos.y += 1;
                } else if dist == (-2, 0) {
                    next_segment.pos.x -= 1;
                } else if dist == (0, -2) {
                    next_segment.pos.y -= 1;
                } else if dist == (-2, 1) || dist == (-1, 2) || dist == (-2, 2) {
                    next_segment.pos.x -= 1;
                    next_segment.pos.y += 1;
                } else if dist == (-1, -2 ) || dist == (-2, -1) || dist == (-2, -2) {
                    next_segment.pos.x -= 1;
                    next_segment.pos.y -= 1;
                } else if dist == (2, -1) || dist == (1, -2) || dist == (2, -2) {
                    next_segment.pos.x += 1;
                    next_segment.pos.y -= 1;
                }
                curr_segment = next_segment;
            }
            self.add_current_tail_position(curr_segment.pos);
        }
    }
}

fn distance_of_curr_and_next(curr_pos: &Position, next_pos: &Position) -> (i64, i64) {
    let x_diff = curr_pos.x - next_pos.x;
    let y_diff = curr_pos.y - next_pos.y;

    (x_diff, y_diff)
}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rope = Rope::new(10);
    let mut knot_tracker = KnotTracker::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        let direction = words.next().unwrap();
        let steps = words.next().unwrap().parse::<i64>().unwrap();
        
        knot_tracker.move_head(&mut rope, direction, steps);
    }

    // println!("{}", rope);
    println!("num of known tail positions {:?}", knot_tracker.known_tail_positions.len());
    // println!("known tail positions {:?}", knot_tracker.known_tail_positions);
}
