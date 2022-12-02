use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum RPSMove {
    Rock,
    Paper,
    Scissors,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
enum RPSOutcome {
    Loss,
    Draw,
    Win
}

#[derive(Debug, Clone, Copy)]
struct Play {
    value: RPSMove,
}

struct Outcome {
    value: RPSOutcome
}

impl Outcome {
    fn build(value: String) -> Outcome {
        match value.as_str() {
            "X" => Outcome { value: RPSOutcome::Loss },
            "Y" => Outcome { value: RPSOutcome::Draw },
            "Z" => Outcome { value: RPSOutcome::Win },
            _ => panic!("Unknown outcome"),
        }
    }
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

    fn calculate_points2(play1: Play, outcome: Outcome) -> i32 {
        match (play1.value, outcome.value) {
            (RPSMove::Paper, RPSOutcome::Loss) => 1,
            (RPSMove::Scissors, RPSOutcome::Loss) => 2,
            (RPSMove::Rock, RPSOutcome::Loss) => 3,
            (RPSMove::Rock, RPSOutcome::Draw) => 4,
            (RPSMove::Paper, RPSOutcome::Draw) => 5,
            (RPSMove::Scissors, RPSOutcome::Draw) => 6,
            (RPSMove::Scissors, RPSOutcome::Win) => 7,
            (RPSMove::Rock, RPSOutcome::Win) => 8,
            (RPSMove::Paper, RPSOutcome::Win) => 9,
            _ => panic!("unknown moves {:?} {:?}", play1.value, outcome.value),
        }
    }
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum_points = 0;
    let mut sum_points2 = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        
        let split: Vec<&str> = line.split(' ').collect();
        let column0 = split[0];
        let column1 = split[1];

        let play0 = Play::build(column0.to_string());
        let play1 = Play::build(column1.to_string());
        let points = RPS::calculate_points(play0.clone(), play1);
        sum_points += points;
        
        let outcome = Outcome::build(column1.to_string());
        let points2 = RPS::calculate_points2(play0, outcome);
        sum_points2 += points2;
    }

    println!("Sum of points: {}", sum_points);
    println!("Sum of points2: {}", sum_points2);
}