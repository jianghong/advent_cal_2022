use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum RPSMove {
    Rock,
    Paper,
    Scissors,
    Unknown,
}

struct Play {
    value: RPSMove,
}

impl Play {
    fn build(value: String) -> Play {
        match value.as_str() {
            "A" => Play { value: RPSMove::Rock },
            "B" => Play { value: RPSMove::Paper },
            "C" => Play { value: RPSMove::Scissors },
            "X" => Play { value: RPSMove::Rock },
            "Y" => Play { value: RPSMove::Paper },
            "Z" => Play { value: RPSMove::Scissors },
            _ => Play { value: RPSMove::Unknown },
        }
    }
}

struct RPS {}

impl RPS {
    fn calculate_points(play1: Play, play2: Play) -> i32 {
        match (play1.value, play2.value) {
            (RPSMove::Paper, RPSMove::Rock) => 1,
            (RPSMove::Scissors, RPSMove::Paper) => 2,
            (RPSMove::Rock, RPSMove::Scissors) => 3,
            (RPSMove::Rock, RPSMove::Rock) => 4,
            (RPSMove::Paper, RPSMove::Paper) => 5,
            (RPSMove::Scissors, RPSMove::Scissors) => 6,
            (RPSMove::Scissors, RPSMove::Rock) => 7,
            (RPSMove::Rock, RPSMove::Paper) => 8,
            (RPSMove::Paper, RPSMove::Scissors) => 9,
            _ => panic!("unknown moves {:?} {:?}", play1.value, play2.value),
        }
    }
}

fn main() {
    // Open the file in read-only mode (ignoring errors).
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum_points = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        
        let split: Vec<&str> = line.split(' ').collect();
        let column0 = split[0];
        let column1 = split[1];

        let play0 = Play::build(column0.to_string());
        let play1 = Play::build(column1.to_string());
        let points = RPS::calculate_points(play0, play1);
        sum_points += points;
    }

    println!("Sum of points: {}", sum_points);
}